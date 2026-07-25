use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalyticsExportManifest {
    pub schema_version: String,
    pub run_id: String,
    pub scenario_id: String,
    pub config_hash: String,
    pub effective_seed: u64,
    pub tick_start: u64,
    pub tick_end: u64,
    pub completeness: String,
    pub warning_codes: Vec<String>,
    pub rows_count: usize,
}

impl Default for AnalyticsExportManifest {
    fn default() -> Self {
        Self {
            schema_version: "1.0".to_string(),
            run_id: "run_default".to_string(),
            scenario_id: "scenario_default".to_string(),
            config_hash: "hash_default".to_string(),
            effective_seed: 0,
            tick_start: 0,
            tick_end: 0,
            completeness: "full".to_string(),
            warning_codes: Vec::new(),
            rows_count: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PopulationAnalyticsRow {
    pub tick: u64,
    pub alive_cells: usize,
    pub stressed_cells: usize,
    pub dead_cells: usize,
    pub total_cells_ever: usize,
    pub births_count: usize,
    pub deaths_count: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BalanceAnalyticsRow {
    pub tick: u64,
    pub total_system_mass: f64,
    pub total_cell_resources: f64,
    pub total_environment_resources: f64,
    pub total_energy_capacity: f64,
    pub total_energy_current: f64,
    pub unaccounted_difference: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineageAnalyticsRow {
    pub tick: u64,
    pub total_lineages: usize,
    pub active_genomes_count: usize,
    pub total_mutations_count: usize,
    pub total_divisions_count: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentAnalyticsRow {
    pub tick: u64,
    pub heat_current: f32,
    pub heat_generated: f32,
    pub waste_current: f32,
    pub waste_generated: f32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsDataset {
    pub manifest: AnalyticsExportManifest,
    pub population: Vec<PopulationAnalyticsRow>,
    pub balance: Vec<BalanceAnalyticsRow>,
    pub lineage: Vec<LineageAnalyticsRow>,
    pub environment: Vec<EnvironmentAnalyticsRow>,
}

impl AnalyticsDataset {
    pub fn new(manifest: AnalyticsExportManifest) -> Self {
        Self {
            manifest,
            population: Vec::new(),
            balance: Vec::new(),
            lineage: Vec::new(),
            environment: Vec::new(),
        }
    }

    pub fn update_rows_count(&mut self) {
        self.manifest.rows_count = self.population.len();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnalyticsExportFiles {
    pub manifest_json_path: PathBuf,
    pub dataset_json_path: PathBuf,
    pub population_csv_path: PathBuf,
    pub balance_csv_path: PathBuf,
    pub lineage_csv_path: PathBuf,
    pub environment_csv_path: PathBuf,
}

pub struct AnalyticsExporter;

impl AnalyticsExporter {
    pub fn to_json(dataset: &AnalyticsDataset) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(dataset)
    }

    pub fn to_population_csv(dataset: &AnalyticsDataset) -> String {
        let mut csv = String::from(
            "tick,alive_cells,stressed_cells,dead_cells,total_cells_ever,births_count,deaths_count\n",
        );
        for row in &dataset.population {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.tick,
                row.alive_cells,
                row.stressed_cells,
                row.dead_cells,
                row.total_cells_ever,
                row.births_count,
                row.deaths_count
            ));
        }
        csv
    }

    pub fn to_balance_csv(dataset: &AnalyticsDataset) -> String {
        let mut csv = String::from(
            "tick,total_system_mass,total_cell_resources,total_environment_resources,total_energy_capacity,total_energy_current,unaccounted_difference\n",
        );
        for row in &dataset.balance {
            csv.push_str(&format!(
                "{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}\n",
                row.tick,
                row.total_system_mass,
                row.total_cell_resources,
                row.total_environment_resources,
                row.total_energy_capacity,
                row.total_energy_current,
                row.unaccounted_difference
            ));
        }
        csv
    }

    pub fn to_lineage_csv(dataset: &AnalyticsDataset) -> String {
        let mut csv = String::from(
            "tick,total_lineages,active_genomes_count,total_mutations_count,total_divisions_count\n",
        );
        for row in &dataset.lineage {
            csv.push_str(&format!(
                "{},{},{},{},{}\n",
                row.tick,
                row.total_lineages,
                row.active_genomes_count,
                row.total_mutations_count,
                row.total_divisions_count
            ));
        }
        csv
    }

    pub fn to_environment_csv(dataset: &AnalyticsDataset) -> String {
        let mut csv =
            String::from("tick,heat_current,heat_generated,waste_current,waste_generated\n");
        for row in &dataset.environment {
            csv.push_str(&format!(
                "{},{:.4},{:.4},{:.4},{:.4}\n",
                row.tick,
                row.heat_current,
                row.heat_generated,
                row.waste_current,
                row.waste_generated
            ));
        }
        csv
    }

    pub fn export_to_dir(
        dataset: &AnalyticsDataset,
        output_dir: &Path,
    ) -> std::io::Result<AnalyticsExportFiles> {
        fs::create_dir_all(output_dir)?;

        let manifest_json_path = output_dir.join("manifest.json");
        let manifest_json =
            serde_json::to_string_pretty(&dataset.manifest).map_err(std::io::Error::other)?;
        let mut file = File::create(&manifest_json_path)?;
        file.write_all(manifest_json.as_bytes())?;

        let dataset_json_path = output_dir.join("dataset.json");
        let dataset_json = Self::to_json(dataset).map_err(std::io::Error::other)?;
        let mut file = File::create(&dataset_json_path)?;
        file.write_all(dataset_json.as_bytes())?;

        let population_csv_path = output_dir.join("population.csv");
        let mut file = File::create(&population_csv_path)?;
        file.write_all(Self::to_population_csv(dataset).as_bytes())?;

        let balance_csv_path = output_dir.join("balance.csv");
        let mut file = File::create(&balance_csv_path)?;
        file.write_all(Self::to_balance_csv(dataset).as_bytes())?;

        let lineage_csv_path = output_dir.join("lineage.csv");
        let mut file = File::create(&lineage_csv_path)?;
        file.write_all(Self::to_lineage_csv(dataset).as_bytes())?;

        let environment_csv_path = output_dir.join("environment.csv");
        let mut file = File::create(&environment_csv_path)?;
        file.write_all(Self::to_environment_csv(dataset).as_bytes())?;

        Ok(AnalyticsExportFiles {
            manifest_json_path,
            dataset_json_path,
            population_csv_path,
            balance_csv_path,
            lineage_csv_path,
            environment_csv_path,
        })
    }
}
