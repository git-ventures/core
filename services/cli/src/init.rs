use anyhow::Error;
use dialoguer::{theme::ColorfulTheme, Input, Select};

#[derive(Debug, Clone)]
pub struct Config {
    pub description: Option<String>,
    pub securities_filing_type: Option<String>,
    pub legal_entity: Option<String>,
    pub open_source: Option<bool>,
    pub product_category: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Init {
    pub config: Config,
}

impl Init {
    pub fn new() -> Self {
        Init {
            config: Config {
                description: None,
                securities_filing_type: None,
                legal_entity: None,
                open_source: None,
                product_category: None,
            },
        }
    }

    pub fn description(&mut self) -> Result<Self, Error> {
        // Set Project Description;
        let description: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Description")
            .interact()?;
        self.config.description = Some(description.to_ascii_uppercase());

        Ok(self.clone())
    }

    pub fn securities_filing_type(&mut self) -> Result<Self, Error> {
        let items = &[
            "Rule 506(b) | Regulation D",
            "Rule 506(c) | Regulation D",
            "Other",
        ];

        // Set Project Security Filing;
        let filing = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select Security Filing Type")
            .default(0)
            .items(items)
            .interact()?;

        // If the filing is `Other`, then
        // prompt the user to input the type;
        if filing == items.len() - 1 {
            let filing_type: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Input Security Filing Type (e.g. Reg CF)")
                .interact()?;
            self.config.filing_type = Some(filing_type.to_ascii_uppercase());
        } else {
            self.config.filing_type = Some(items[filing].to_string().to_ascii_uppercase());
        }

        Ok(self.clone())
    }

    
    pub fn legal_entity(&mut self) -> Result<Self, Error> {
        
    }
    
    pub fn open_source(&mut self) -> Result<Self, Error> {
        
    }
    
    pub fn product_category(&mut self) -> Result<Self, Error> {
        
    }

    
}
