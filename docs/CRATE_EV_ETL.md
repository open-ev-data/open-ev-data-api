# ev-etl

ETL pipeline for processing OpenEV Data vehicle datasets.

## Overview

This crate provides a command-line tool for transforming layered JSON vehicle data from the dataset repository into multiple output formats.

## Usage

```bash
ev-etl --input ../open-ev-data-dataset/src --output ./output --formats json,sqlite
```

## Output Formats

- **JSON**: Canonical vehicle array with metadata
- **SQLite**: Normalized relational database
- **PostgreSQL**: DDL + INSERT statements
- **CSV**: Flattened tabular format
- **XML**: Hierarchical structure

## Options

- `--input, -i`: Input directory containing vehicle JSON files
- `--output, -o`: Output directory for artifacts (default: ./output)
- `--formats, -f`: Comma-separated output formats (default: json,sqlite)
- `--validate-only`: Validate without generating output
- `--verbose, -v`: Enable debug logging
