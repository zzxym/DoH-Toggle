use std::process::Command;

#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

#[derive(Debug, PartialEq)]
pub enum DohStatus {
    Enabled,
    Disabled,
    Unknown,
}

#[derive(Debug)]
pub enum Browser {
    Chrome,
    Edge,
}

pub fn get_doh_status(browser: Browser) -> DohStatus {
    #[cfg(target_os = "windows")] {
        return match browser {
            Browser::Chrome => get_chrome_doh_status_windows(),
            Browser::Edge => get_edge_doh_status_windows(),
        };
    }
    
    #[cfg(target_os = "macos")] {
        return match browser {
            Browser::Chrome => get_chrome_doh_status_macos(),
            Browser::Edge => get_edge_doh_status_macos(),
        };
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos")))] {
        return DohStatus::Unknown;
    }
}

pub fn set_doh_status(browser: Browser, enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")] {
        return match browser {
            Browser::Chrome => set_chrome_doh_status_windows(enabled),
            Browser::Edge => set_edge_doh_status_windows(enabled),
        };
    }
    
    #[cfg(target_os = "macos")] {
        return match browser {
            Browser::Chrome => set_chrome_doh_status_macos(enabled),
            Browser::Edge => set_edge_doh_status_macos(enabled),
        };
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos")))] {
        return Err("Unsupported platform".to_string());
    }
}

#[cfg(target_os = "windows")]
fn get_chrome_doh_status_windows() -> DohStatus {
    match RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags("SOFTWARE\\Policies\\Google\\Chrome", KEY_READ)
    {
        Ok(key) => match key.get_value("DnsOverHttpsMode") {
            Ok(value) => {
                if value == "off" {
                    DohStatus::Disabled
                } else {
                    DohStatus::Enabled
                }
            }
            Err(_) => DohStatus::Unknown,
        },
        Err(_) => DohStatus::Unknown,
    }
}

#[cfg(target_os = "windows")]
fn get_edge_doh_status_windows() -> DohStatus {
    match RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags("SOFTWARE\\Policies\\Microsoft\\Edge", KEY_READ)
    {
        Ok(key) => match key.get_value("DnsOverHttpsMode") {
            Ok(value) => {
                if value == "off" {
                    DohStatus::Disabled
                } else {
                    DohStatus::Enabled
                }
            }
            Err(_) => DohStatus::Unknown,
        },
        Err(_) => DohStatus::Unknown,
    }
}

#[cfg(target_os = "windows")]
fn set_chrome_doh_status_windows(enabled: bool) -> Result<(), String> {
    let value = if enabled { "automatic" } else { "off" };
    
    match RegKey::predef(HKEY_LOCAL_MACHINE)
        .create_subkey("SOFTWARE\\Policies\\Google\\Chrome")
    {
        Ok((key, _)) => match key.set_value("DnsOverHttpsMode", &value) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to set Chrome DOH status: {:?}", e)),
        },
        Err(e) => Err(format!("Failed to create Chrome registry key: {:?}", e)),
    }
}

#[cfg(target_os = "windows")]
fn set_edge_doh_status_windows(enabled: bool) -> Result<(), String> {
    let value = if enabled { "automatic" } else { "off" };
    
    match RegKey::predef(HKEY_LOCAL_MACHINE)
        .create_subkey("SOFTWARE\\Policies\\Microsoft\\Edge")
    {
        Ok((key, _)) => match key.set_value("DnsOverHttpsMode", &value) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to set Edge DOH status: {:?}", e)),
        },
        Err(e) => Err(format!("Failed to create Edge registry key: {:?}", e)),
    }
}

#[cfg(target_os = "macos")]
fn get_chrome_doh_status_macos() -> DohStatus {
    let output = Command::new("defaults")
        .args(["read", "com.google.Chrome", "DnsOverHttpsMode"])
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if value == "off" {
                    DohStatus::Disabled
                } else {
                    DohStatus::Enabled
                }
            } else {
                DohStatus::Unknown
            }
        },
        Err(_) => DohStatus::Unknown,
    }
}

#[cfg(target_os = "macos")]
fn get_edge_doh_status_macos() -> DohStatus {
    let output = Command::new("defaults")
        .args(["read", "com.microsoft.Edge", "DnsOverHttpsMode"])
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if value == "off" {
                    DohStatus::Disabled
                } else {
                    DohStatus::Enabled
                }
            } else {
                DohStatus::Unknown
            }
        },
        Err(_) => DohStatus::Unknown,
    }
}

#[cfg(target_os = "macos")]
fn set_chrome_doh_status_macos(enabled: bool) -> Result<(), String> {
    let value = if enabled { "automatic" } else { "off" };
    
    let output = Command::new("defaults")
        .args(["write", "com.google.Chrome", "DnsOverHttpsMode", "-string", value])
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err("Failed to set Chrome DOH status".to_string())
            }
        },
        Err(e) => Err(format!("Failed to execute defaults command: {:?}", e)),
    }
}

#[cfg(target_os = "macos")]
fn set_edge_doh_status_macos(enabled: bool) -> Result<(), String> {
    let value = if enabled { "automatic" } else { "off" };
    
    let output = Command::new("defaults")
        .args(["write", "com.microsoft.Edge", "DnsOverHttpsMode", "-string", value])
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err("Failed to set Edge DOH status".to_string())
            }
        },
        Err(e) => Err(format!("Failed to execute defaults command: {:?}", e)),
    }
}