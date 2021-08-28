pub struct Communication {
    current_wifi_ssid: Option<String>,
}

impl Communication {
    pub fn connect_wifi(&mut self, wifi_ssd: &str) {
        self.current_wifi_ssid = Some(wifi_ssd.into());
        println!("Conectado na wifi {}", wifi_ssd);
    }
    pub fn send_hello_message(&self) {
        self.send_message("Hello!");
    }
    fn send_message(&self, message: &str) {
        match &self.current_wifi_ssid {
            Some(ssid) => println!("Enviando mensagem '{}' pela wifi '{}'", message, ssid),
            None => println!("Não conectado"),
        }
    }
}

impl Default for Communication {
    fn default() -> Self {
        Communication {
            current_wifi_ssid: None,
        }
    }
}
