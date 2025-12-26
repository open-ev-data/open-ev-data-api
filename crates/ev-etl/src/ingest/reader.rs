use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde_json::Value;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct VehicleFile {
    pub path: PathBuf,
    pub make_slug: String,
    pub model_slug: String,
    pub year: Option<u16>,
    pub file_type: FileType,
    pub content: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    ModelBase,
    YearBase,
    Variant,
}

pub fn scan_directory(input_dir: &Path) -> Result<Vec<VehicleFile>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(input_dir)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
    {
        let path = entry.path();

        if path
            .file_name()
            .is_some_and(|n| n.to_string_lossy().starts_with('.'))
        {
            continue;
        }

        if let Some(vehicle_file) = parse_vehicle_file(path, input_dir)? {
            files.push(vehicle_file);
        }
    }

    files.sort_by(|a, b| {
        a.make_slug
            .cmp(&b.make_slug)
            .then(a.model_slug.cmp(&b.model_slug))
            .then(a.year.cmp(&b.year))
            .then(a.file_type.cmp(&b.file_type))
    });

    Ok(files)
}

impl Ord for FileType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_order = match self {
            Self::ModelBase => 0,
            Self::YearBase => 1,
            Self::Variant => 2,
        };
        let other_order = match other {
            Self::ModelBase => 0,
            Self::YearBase => 1,
            Self::Variant => 2,
        };
        self_order.cmp(&other_order)
    }
}

impl PartialOrd for FileType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_vehicle_file(path: &Path, base_dir: &Path) -> Result<Option<VehicleFile>> {
    let relative = path
        .strip_prefix(base_dir)
        .context("Failed to get relative path")?;

    let components: Vec<_> = relative
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();

    if components.len() < 2 {
        return Ok(None);
    }

    let make_slug = components[0].to_string();
    let model_slug = components[1].to_string();

    let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

    let (year, file_type) = if file_name == "base" && components.len() == 3 {
        (None, FileType::ModelBase)
    } else if components.len() == 4 {
        let year_str = components[2];
        let year: u16 = year_str.parse().unwrap_or(0);

        if year == 0 {
            return Ok(None);
        }

        let expected_base_name = format!("{}_{}", make_slug, model_slug);
        if file_name == expected_base_name
            || file_name.starts_with(&format!("{}_", expected_base_name))
        {
            if file_name == expected_base_name {
                (Some(year), FileType::YearBase)
            } else {
                (Some(year), FileType::Variant)
            }
        } else if !file_name.contains('_') {
            (Some(year), FileType::YearBase)
        } else {
            (Some(year), FileType::Variant)
        }
    } else {
        return Ok(None);
    };

    let content_str =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read: {:?}", path))?;

    let content: Value = serde_json::from_str(&content_str)
        .with_context(|| format!("Failed to parse: {:?}", path))?;

    Ok(Some(VehicleFile {
        path: path.to_path_buf(),
        make_slug,
        model_slug,
        year,
        file_type,
        content,
    }))
}
