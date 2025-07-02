/*
 * 📦 Coffee Bean Package Roastery - Package Management System 📦
 * 
 * @package_master: Khushi Motwani 
 * @vision: "npm but for coffee beans" ☕📦
 * @complexity_rating: "Ambitious but totally doable" ✨
 * 
 * Welcome to the Coffee Package Roastery - where developers can 
 * share and discover amazing coffee-themed packages! 
 * 
 * Every function here dreams of the day when there's a thriving 
 * ecosystem of Brewco packages! 💖
 * 
 * @khushi_dream: "One day there will be thousands of coffee packages!"
 * - Khushi 💖
 */

// src/coffee_package_roastery.rs - The Coffee Bean Package Roastery Supply Chain ☕

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use crate::espresso_errors::{CoffeeSpillReport, SpillType};
use crate::coffee_io::CoffeeFileBrewery;

/// The Coffee Bean Package Roastery - manages package installation and dependencies
pub struct CoffeeBeanPackageRoastery {
    roastery_manifest: RoasteryManifest,
    installed_beans: HashMap<String, InstalledCoffeeBean>,
    roastery_sources: Vec<RoasterySource>,
    local_bean_storage: PathBuf,
}

/// Roastery Manifest - like package.json but for coffee beans
#[derive(Serialize, Deserialize, Clone)]
pub struct RoasteryManifest {
    pub roastery_name: String,
    pub roastery_version: String,
    pub roastery_description: String,
    pub head_barista: String, // author
    pub coffee_license: String,
    pub required_bean_dependencies: HashMap<String, String>, // name -> version
    pub dev_brewing_dependencies: HashMap<String, String>,   // dev dependencies
    pub brewing_scripts: HashMap<String, String>,            // custom scripts
    pub coffee_keywords: Vec<String>,
    pub roastery_homepage: Option<String>,
    pub bean_repository: Option<String>,
}

/// An installed coffee bean package
#[derive(Clone)]
pub struct InstalledCoffeeBean {
    pub bean_name: String,
    pub bean_version: String,
    pub bean_origin: String,        // where it was downloaded from
    pub installation_path: PathBuf,
    pub brewing_dependencies: Vec<String>,
    pub install_time: std::time::SystemTime,
}

/// Roastery Source - where to find coffee bean packages
#[derive(Clone)]
pub struct RoasterySource {
    pub source_name: String,
    pub source_url: String,
    pub is_secure_source: bool,
    pub authentication_token: Option<String>,
}

/// Coffee Bean Package Info - metadata about available packages
#[derive(Serialize, Deserialize, Clone)]
pub struct CoffeeBeanPackageInfo {
    pub bean_name: String,
    pub available_versions: Vec<String>,
    pub latest_version: String,
    pub bean_description: String,
    pub roastery_author: String,
    pub download_count: u64,
    pub brewing_dependencies: HashMap<String, String>,
    pub roastery_homepage: Option<String>,
    pub coffee_keywords: Vec<String>,
}

impl CoffeeBeanPackageRoastery {
    pub fn new_roastery_manager() -> Result<Self, CoffeeSpillReport> {
        let local_bean_storage = PathBuf::from("./coffee_beans");
        
        let default_roastery_sources = vec![
            RoasterySource {
                source_name: "The Central Coffee Bean Repository".to_string(),
                source_url: "https://beans.brewco.org".to_string(),
                is_secure_source: true,
                authentication_token: None,
            },
            RoasterySource {
                source_name: "Community Coffee Roastery".to_string(),
                source_url: "https://community.brewco.org".to_string(),
                is_secure_source: true,
                authentication_token: None,
            },
        ];
        
        // Try to load existing manifest
        let roastery_manifest = if Path::new("roastery.json").exists() {
            Self::load_roastery_manifest()?
        } else {
            Self::create_default_manifest()
        };
        
        Ok(CoffeeBeanPackageRoastery {
            roastery_manifest,
            installed_beans: HashMap::new(),
            roastery_sources: default_roastery_sources,
            local_bean_storage,
        })
    }
    
    /// Initialize a new coffee roastery project
    pub fn brew_new_roastery(
        roastery_name: &str,
        head_barista: &str,
        description: &str
    ) -> Result<(), CoffeeSpillReport> {
        let manifest = RoasteryManifest {
            roastery_name: roastery_name.to_string(),
            roastery_version: "1.0.0".to_string(),
            roastery_description: description.to_string(),
            head_barista: head_barista.to_string(),
            coffee_license: "Coffee-License-1.0".to_string(),
            required_bean_dependencies: HashMap::new(),
            dev_brewing_dependencies: HashMap::new(),
            brewing_scripts: Self::default_brewing_scripts(),
            coffee_keywords: vec!["coffee".to_string(), "brewco".to_string()],
            roastery_homepage: None,
            bean_repository: None,
        };
        
        Self::save_roastery_manifest(&manifest)?;
        
        // Create directory structure
        std::fs::create_dir_all("coffee_beans").map_err(|e| {
            CoffeeSpillReport::new_brewing_disaster(
                SpillType::OverExtraction,
                0, 0,
                &format!("Failed to create coffee beans directory: {}", e)
            )
        })?;
        
        std::fs::create_dir_all("roastery").map_err(|e| {
            CoffeeSpillReport::new_brewing_disaster(
                SpillType::OverExtraction,
                0, 0,
                &format!("Failed to create roastery directory: {}", e)
            )
        })?;
        
        // Create a sample coffee file
        let sample_coffee = r#"🎀 Welcome to your new Coffee Roastery! ☕
🎀 Generated by the Coffee Bean Package Roastery

beans greeting pour_in "Hello from your new roastery!"
beans roastery_name pour_in "My Coffee Roastery"

pourout greeting
pourout "Roastery Name:", roastery_name

🎀 Happy brewing! ☕✨
"#;
        
                    CoffeeFileBrewery::pour_recipe_to_file("main.brewco", sample_coffee)?;
        
        println!("☕ New Coffee Roastery '{}' brewed successfully!", roastery_name);
        println!("📁 Files created:");
        println!("   - roastery.json (roastery manifest)");
                    println!("   - main.brewco (sample coffee file)");
        println!("   - coffee_beans/ (dependencies directory)");
        println!("   - roastery/ (project source directory)");
        
        Ok(())
    }
    
    /// Install a coffee bean package
    pub fn install_coffee_bean(
        &mut self,
        bean_name: &str,
        bean_version: Option<&str>
    ) -> Result<(), CoffeeSpillReport> {
        println!("☕ Brewing installation of coffee bean '{}'...", bean_name);
        
        // Find the bean in roastery sources
        let bean_info = self.search_coffee_bean(bean_name)?;
        let version_to_install = bean_version.unwrap_or(&bean_info.latest_version);
        
        // Check if already installed
        if let Some(existing) = self.installed_beans.get(bean_name) {
            if existing.bean_version == version_to_install {
                println!("☕ Coffee bean '{}' version {} is already perfectly brewed!", bean_name, version_to_install);
                return Ok(());
            }
        }
        
        // Install dependencies first
        for (dep_name, dep_version) in &bean_info.brewing_dependencies {
            if !self.installed_beans.contains_key(dep_name) {
                println!("☕ Installing dependency coffee bean '{}'...", dep_name);
                self.install_coffee_bean(dep_name, Some(dep_version))?;
            }
        }
        
        // Download and install the bean
        let installation_path = self.download_coffee_bean(bean_name, version_to_install)?;
        
        let installed_bean = InstalledCoffeeBean {
            bean_name: bean_name.to_string(),
            bean_version: version_to_install.to_string(),
            bean_origin: "Central Coffee Bean Repository".to_string(),
            installation_path,
            brewing_dependencies: bean_info.brewing_dependencies.keys().cloned().collect(),
            install_time: std::time::SystemTime::now(),
        };
        
        self.installed_beans.insert(bean_name.to_string(), installed_bean);
        
        // Update roastery manifest
        self.roastery_manifest.required_bean_dependencies.insert(
            bean_name.to_string(),
            version_to_install.to_string()
        );
        
        Self::save_roastery_manifest(&self.roastery_manifest)?;
        
        println!("☕ Coffee bean '{}' version {} successfully brewed and installed!", bean_name, version_to_install);
        Ok(())
    }
    
    /// Search for a coffee bean in roastery sources
    fn search_coffee_bean(&self, bean_name: &str) -> Result<CoffeeBeanPackageInfo, CoffeeSpillReport> {
        // For now, return mock data - in real implementation, this would query the sources
        Ok(CoffeeBeanPackageInfo {
            bean_name: bean_name.to_string(),
            available_versions: vec!["1.0.0".to_string(), "1.1.0".to_string()],
            latest_version: "1.1.0".to_string(),
            bean_description: format!("A delicious {} coffee bean package", bean_name),
            roastery_author: "Coffee Master".to_string(),
            download_count: 1337,
            brewing_dependencies: HashMap::new(),
            roastery_homepage: Some(format!("https://beans.brewco.org/{}", bean_name)),
            coffee_keywords: vec!["coffee".to_string(), "utility".to_string()],
        })
    }
    
    /// Download a coffee bean package
    fn download_coffee_bean(&self, bean_name: &str, version: &str) -> Result<PathBuf, CoffeeSpillReport> {
        let bean_dir = self.local_bean_storage.join(format!("{}@{}", bean_name, version));
        
        // Create the directory
        std::fs::create_dir_all(&bean_dir).map_err(|e| {
            CoffeeSpillReport::new_brewing_disaster(
                SpillType::OverExtraction,
                0, 0,
                &format!("Failed to create bean directory: {}", e)
            )
        })?;
        
        // For now, create a mock bean file - in real implementation, this would download from sources
        let mock_bean_content = format!(r#"🎀 {} Coffee Bean Package ☕
🎀 Version: {}
🎀 Auto-downloaded from Central Coffee Bean Repository

beans {}_greeting pour_in "Hello from {} bean!"
beans {}_version pour_in "{}"

brew get_{}_info() {{
    pourout "Bean:", "{}"
    pourout "Version:", {}_version
    return {}_greeting
}}

🎀 Exported functions and variables are available for import! ☕
"#, bean_name, version, bean_name, bean_name, bean_name, version, bean_name, bean_name, bean_name, bean_name);
        
        let bean_file = bean_dir.join("index.brewco");
        CoffeeFileBrewery::pour_recipe_to_file(&bean_file.to_string_lossy(), &mock_bean_content)?;
        
        Ok(bean_dir)
    }
    
    /// List installed coffee beans
    pub fn list_brewed_beans(&self) -> Vec<&InstalledCoffeeBean> {
        self.installed_beans.values().collect()
    }
    
    /// Uninstall a coffee bean
    pub fn remove_coffee_bean(&mut self, bean_name: &str) -> Result<(), CoffeeSpillReport> {
        if let Some(bean) = self.installed_beans.remove(bean_name) {
            // Remove from filesystem
            if bean.installation_path.exists() {
                std::fs::remove_dir_all(&bean.installation_path).map_err(|e| {
                    CoffeeSpillReport::new_brewing_disaster(
                        SpillType::OverExtraction,
                        0, 0,
                        &format!("Failed to remove bean directory: {}", e)
                    )
                })?;
            }
            
            // Remove from manifest
            self.roastery_manifest.required_bean_dependencies.remove(bean_name);
            Self::save_roastery_manifest(&self.roastery_manifest)?;
            
            println!("☕ Coffee bean '{}' has been disposed of properly!", bean_name);
            Ok(())
        } else {
            Err(CoffeeSpillReport::new_brewing_disaster(
                SpillType::BeanNotFound,
                0, 0,
                &format!("Coffee bean '{}' is not installed in this roastery", bean_name)
            ))
        }
    }
    
    /// Load roastery manifest from file
    fn load_roastery_manifest() -> Result<RoasteryManifest, CoffeeSpillReport> {
        let manifest_content = CoffeeFileBrewery::sip_entire_recipe("roastery.json")?;
        serde_json::from_str(&manifest_content).map_err(|e| {
            CoffeeSpillReport::new_brewing_disaster(
                SpillType::IncompleteRecipe,
                0, 0,
                &format!("Failed to parse roastery manifest: {}", e)
            )
        })
    }
    
    /// Save roastery manifest to file
    fn save_roastery_manifest(manifest: &RoasteryManifest) -> Result<(), CoffeeSpillReport> {
        let manifest_json = serde_json::to_string_pretty(manifest).map_err(|e| {
            CoffeeSpillReport::new_brewing_disaster(
                SpillType::OverExtraction,
                0, 0,
                &format!("Failed to serialize roastery manifest: {}", e)
            )
        })?;
        
        CoffeeFileBrewery::pour_recipe_to_file("roastery.json", &manifest_json)
    }
    
    /// Create default manifest
    fn create_default_manifest() -> RoasteryManifest {
        RoasteryManifest {
            roastery_name: "my-coffee-roastery".to_string(),
            roastery_version: "1.0.0".to_string(),
            roastery_description: "A new Brewco roastery project".to_string(),
            head_barista: "Coffee Enthusiast".to_string(),
            coffee_license: "Coffee-License-1.0".to_string(),
            required_bean_dependencies: HashMap::new(),
            dev_brewing_dependencies: HashMap::new(),
            brewing_scripts: Self::default_brewing_scripts(),
            coffee_keywords: vec!["coffee".to_string()],
            roastery_homepage: None,
            bean_repository: None,
        }
    }
    
    /// Default brewing scripts
    fn default_brewing_scripts() -> HashMap<String, String> {
        let mut scripts = HashMap::new();
        scripts.insert("start".to_string(), "brew main.brewco".to_string());
        scripts.insert("test".to_string(), "brew test/*.brewco".to_string());
        scripts.insert("build".to_string(), "brew --optimize main.brewco".to_string());
        scripts
    }
}

/// Native functions for Brewco package management
pub fn native_install_bean(args: Vec<crate::interpreter::Value>) -> Result<crate::interpreter::Value, crate::interpreter::ControlFlow> {
    if args.is_empty() {
        return Err(crate::interpreter::ControlFlow::RuntimeError(
            "install_bean() expects at least 1 argument (bean name)".to_string()
        ));
    }
    
    match &args[0] {
        crate::interpreter::Value::String(bean_name) => {
            let version = if args.len() > 1 {
                match &args[1] {
                    crate::interpreter::Value::String(v) => Some(v.as_str()),
                    _ => None,
                }
            } else {
                None
            };
            
            // For now, just return success - real implementation would use roastery manager
            println!("☕ Installing coffee bean '{}'...", bean_name);
            Ok(crate::interpreter::Value::Boolean(true))
        }
        _ => Err(crate::interpreter::ControlFlow::RuntimeError(
            "install_bean() expects a string bean name".to_string()
        ))
    }
}

pub fn native_list_brewed_beans(args: Vec<crate::interpreter::Value>) -> Result<crate::interpreter::Value, crate::interpreter::ControlFlow> {
    // Mock implementation - return some sample installed beans
    let mock_beans = vec![
        "espresso_utils".to_string(),
        "coffee_math".to_string(),
        "brewing_helpers".to_string(),
    ];
    
    let values: Vec<crate::interpreter::Value> = mock_beans
        .into_iter()
        .map(|b| crate::interpreter::Value::String(b))
        .collect();
        
    Ok(crate::interpreter::Value::Array(values))
} 