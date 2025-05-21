use serde::{Deserialize, Serialize};
use std::{fs, path::Path, process::Command};

#[derive(Debug, Deserialize, Serialize)]
pub struct AnarchConfig {
    pub pacman: Option<Vec<String>>,
    pub aur: Option<Vec<String>>,
    pub flatpak: Option<Vec<String>>,
}

impl AnarchConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let toml_str = fs::read_to_string(path)?;
        let cfg: AnarchConfig = toml::from_str(&toml_str)?;
        Ok(cfg)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = Path::new(path.as_ref()).parent() {
            fs::create_dir_all(parent)?;
        }

        let toml_str = toml::to_string_pretty(&self)?;
        fs::write(path, toml_str)?;
        Ok(())
    }

    pub fn generate_template() -> Self {
        AnarchConfig {
            pacman: Some(vec![
                "base-devel".to_string(),
                "git".to_string(),
                "firefox".to_string(),
            ]),
            aur: Some(vec![
                "paru-bin".to_string(),
                "visual-studio-code-bin".to_string(),
            ]),
            flatpak: Some(vec![
                "com.spotify.Client".to_string(),
                "org.telegram.desktop".to_string(),
            ]),
        }
    }
}

pub struct PackageManager {
    pacman: Vec<String>,
    aur: Vec<String>,
    flatpak: Vec<String>,
}

impl From<&AnarchConfig> for PackageManager {
    fn from(cfg: &AnarchConfig) -> Self {
        PackageManager {
            pacman: cfg.pacman.clone().unwrap_or_default(),
            aur: cfg.aur.clone().unwrap_or_default(),
            flatpak: cfg.flatpak.clone().unwrap_or_default(),
        }
    }
}

impl PackageManager {
    pub fn extract_installed_packages() -> Result<AnarchConfig, Box<dyn std::error::Error>> {
        let pacman_packages = Self::get_pacman_packages()?;
        let aur_packages = Self::get_aur_packages()?;
        let flatpak_packages = Self::get_flatpak_packages()?;

        Ok(AnarchConfig {
            pacman: Some(pacman_packages),
            aur: Some(aur_packages),
            flatpak: Some(flatpak_packages),
        })
    }

    fn get_pacman_packages() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let output = Command::new("pacman").args(["-Qn"]).output()?;

        if !output.status.success() {
            return Err("Failed to get pacman packages".into());
        }

        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout
            .lines()
            .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
            .filter(|pkg| !pkg.is_empty())
            .collect())
    }

    fn get_aur_packages() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let output = Command::new("pacman")
            .args(["-Qm"]) // Query foreign packages (AUR)
            .output()?;

        if !output.status.success() {
            return Err("Failed to get AUR packages".into());
        }

        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout
            .lines()
            .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
            .filter(|pkg| !pkg.is_empty())
            .collect())
    }

    fn get_flatpak_packages() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // Check if flatpak is installed
        if !Command::new("which").arg("flatpak").status()?.success() {
            return Ok(Vec::new());
        }

        let output = Command::new("flatpak")
            .args(["list", "--app", "--columns=application"])
            .output()?;

        if !output.status.success() {
            return Err("Failed to get Flatpak packages".into());
        }

        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|pkg| !pkg.is_empty())
            .collect())
    }

    pub fn install_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.pacman.is_empty() {
            println!("Installing Pacman packages: {:?}", self.pacman);
            self.install_pacman_packages()?;
        }

        if !self.aur.is_empty() {
            println!("Installing AUR packages: {:?}", self.aur);
            self.install_aur_packages()?;
        }

        if !self.flatpak.is_empty() {
            println!("Installing Flatpak packages: {:?}", self.flatpak);
            self.install_flatpak_packages()?;
        }

        Ok(())
    }

    fn install_pacman_packages(&self) -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("--needed")
            .arg("--noconfirm")
            .args(&self.pacman)
            .status()?;

        if !status.success() {
            return Err("Failed to install Pacman packages".into());
        }

        Ok(())
    }

    fn install_aur_packages(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !Command::new("which").arg("paru").status()?.success() {
            return Err("Failed to install AUR packages".into());
        }

        let status = Command::new("paru")
            .arg("-S")
            .arg("--needed")
            .arg("--noconfirm")
            .args(&self.aur)
            .status()?;

        if !status.success() {
            return Err("Failed to install AUR packages".into());
        }

        Ok(())
    }

    fn install_flatpak_packages(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !Command::new("which").arg("flatpak").status()?.success() {
            println!("Installing Flatpak...");
            let status = Command::new("sudo")
                .args(&["pacman", "-S", "--needed", "--noconfirm", "flatpak"])
                .status()?;

            if !status.success() {
                return Err("Failed to install Flatpak".into());
            }
        }

        Command::new("flatpak")
            .args(&[
                "remote-add",
                "--if-not-exists",
                "flathub",
                "https://dl.flathub.org/repo/flathub.flatpakrepo",
            ])
            .status()?;

        for package in &self.flatpak {
            let status = Command::new("flatpak")
                .args(&["install", "--noninteractive", "flathub", package])
                .status()?;

            if !status.success() {
                return Err(format!("Failed to install Flatpak package: {}", package).into());
            }
        }

        Ok(())
    }
}
