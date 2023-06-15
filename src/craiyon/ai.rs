use std::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    collections::HashMap,
    error::Error,
    fmt::Display,
};

use clap::ValueEnum;
use image::DynamicImage;
use lazy_static::lazy_static;
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Response,
};
use serde::{Deserialize, Serialize};

const URL_V3: &str = "https://api.craiyon.com/v3";
// const URL_V2: &str = "https://api.craiyon.com/draw"; // deprecated
const URL_V1: &str = "https://backend.craiyon.com/generate";
const URL_IMAGE: &str = "https://img.craiyon.com";
const MODEL_VER: &str = "35s5hfwn9n78gb06";

lazy_static! {
    static ref HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    };
}

async fn send_req(
    url: &str,
    json: &HashMap<&str, Option<&str>>,
) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .json(json)
        .headers(HEADERS.clone()) // FIXME
        .send()
        .await?;

    Ok(res)
}

pub struct Model<'a> {
    model: ModelType,
    version: Api,
    api_token: Option<&'a str>,
}

#[allow(dead_code)]
impl<'a> Model<'a> {
    pub fn new() -> Self {
        Self {
            model: Default::default(),
            version: Default::default(),
            api_token: None,
        }
    }

    pub fn version(mut self, ver: Api) -> Self {
        self.version = ver;
        self
    }

    pub fn api_token(mut self, api_token: Option<&'a str>) -> Self {
        self.api_token = api_token;
        self
    }

    pub fn model_type(mut self, mod_type: ModelType) -> Self {
        self.model = mod_type;
        self
    }

    pub fn from(model: ModelType, version: Api) -> Self {
        Self {
            model,
            version,
            api_token: None,
        }
    }

    #[allow(dead_code)]
    pub async fn from_prompt(
        &self,
        prompt: &str,
        num_images: usize,
    ) -> Result<Vec<DynamicImage>, Box<dyn Error>> {
        Ok(self.generate(prompt, "", num_images).await?)
    }

    #[allow(dead_code)]
    pub async fn generate(
        &self,
        prompt: &str,
        negative_prompt: &str,
        num_images: usize,
    ) -> Result<Vec<DynamicImage>, Box<dyn Error>> {
        if num_images > 9 {
            panic!("Argument `num_images` has to be within the range of 0..9")
        }

        let model = &self.model.to_string();

        let data = match self.version {
            Api::V1 => HashMap::from([("prompt", Some(prompt))]),

            Api::V3 => HashMap::from([
                ("prompt", Some(prompt)),
                ("negative_prompt", Some(negative_prompt)),
                ("model", Some(model)),
                ("token", self.api_token),
                ("version", Some(MODEL_VER)),
            ]),
        };

        let response = send_req(&self.version.to_string(), &data).await?;

        let res: CraiyonResponse = response.json().await?;

        let image_urls: Vec<String> = res
            .images
            .iter()
            .take(num_images)
            .map(|image| format!("{}/{}", URL_IMAGE, image))
            .collect();

        let mut image_buf: Vec<DynamicImage> = Vec::with_capacity(image_urls.len());

        for image_url in image_urls {
            let pixels = reqwest::blocking::get(image_url)?.bytes()?.to_vec();

            let image = image::load_from_memory(&pixels)?;

            image_buf.push(image);
        }
        Ok(image_buf)
    }
}

impl Default for Model<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Variants of craiyon::Model
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Ord, ValueEnum)]
pub enum ModelType {
    Art,
    Drawing,
    Photo,
    #[default]
    General,
}

impl Display for ModelType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match self {
            ModelType::Art => f.write_str("art"),
            ModelType::Drawing => f.write_str("drawing"),
            ModelType::Photo => f.write_str("photo"),
            ModelType::General => f.write_str("none"),
        }
    }
}

/// API Versions for craiyon.com
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Ord, ValueEnum)]
pub enum Api {
    #[value(name = "1")]
    V1,
    // V2, // deprecated
    #[default]
    #[value(name = "3")]
    V3,
}

impl Display for Api {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match self {
            Api::V1 => f.write_str(URL_V1),
            // Api::V2 => f.write_str(URL_V2),
            Api::V3 => f.write_str(URL_V3),
        }
    }
}

/// Response Deserializer
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CraiyonResponse {
    pub images: Vec<String>,
}
