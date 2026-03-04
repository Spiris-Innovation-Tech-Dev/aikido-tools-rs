use anyhow::Result;
use clap::ValueEnum;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Pretty,
    Json,
    Toon,
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "pretty" => Ok(OutputFormat::Pretty),
            "json" => Ok(OutputFormat::Json),
            "toon" => Ok(OutputFormat::Toon),
            _ => Err(anyhow::anyhow!(
                "Invalid output format '{}'. Valid: pretty, json, toon",
                s
            )),
        }
    }
}

pub trait Formattable: Serialize {
    fn format_pretty(&self) -> Result<String>;

    fn format_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    fn format_toon(&self) -> Result<String>
    where
        Self: Sized,
    {
        let options = toon_rs::Options::default();
        Ok(toon_rs::encode_to_string(self, &options)?)
    }

    fn format(&self, format: OutputFormat) -> Result<String>
    where
        Self: Sized,
    {
        match format {
            OutputFormat::Pretty => self.format_pretty(),
            OutputFormat::Json => self.format_json(),
            OutputFormat::Toon => self.format_toon(),
        }
    }
}
