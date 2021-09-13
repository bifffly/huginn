use dns_lookup::lookup_host;
use std::io::{Read, Write};
use std::net::TcpStream;
use url::Url;
use vgtk::{Component, gtk, UpdateAction, VNode};
use vgtk::ext::*;
use vgtk::lib::gtk::*;

use crate::*;
use crate::parser::*;

impl Component for HuginnDisplay {
    type Message = Msg;
    type Properties = Self;

    fn create(props: Self) -> Self {
        return props;
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        return UpdateAction::Render;
    }

    fn update(&mut self, msg: Msg) -> UpdateAction<Self> {
        match msg {
            _ => {}
        };
        return UpdateAction::None;
    }

    fn view(&self) -> VNode<Self> {
        let url = self.url.clone();
        let response = send_request(url);
        let components = odin_to_components(response);
        gtk! {
            <ScrolledWindow vexpand=true>
                <Box orientation=Orientation::Vertical spacing=10>
                    {components.iter().map(HuginnComponent::render)}
                </Box>
            </ScrolledWindow>
        }
    }
}

impl HuginnComponent {
    pub fn new<S: ToString>(string: S) -> Self {
        let label = string.to_string();
        return Self {label};
    } 

    fn render(&self) -> VNode<HuginnDisplay> {
        let label = self.label.clone();

        gtk! {
            <Label xalign=0.0 yalign=0.0 markup=label/>
        }       
    }
}

fn send_request(url_string: String) -> String {
    let url_result = Url::parse(&url_string);
    if url_result.is_ok() {
        let url = url_result.ok().unwrap();
        let ipaddr = lookup_host(url.host_str().unwrap()).unwrap()[0];
        let mut stream = TcpStream::connect(ipaddr.to_string() + &":1866").unwrap();
        let reqstr = "odin\tpull\t".to_owned() + &url.path();
        stream.write(reqstr.as_bytes()).expect("Could not write to stream");
        let mut buffer = [0; 4096];
        stream.read(&mut buffer).unwrap();
        let mut v = vec![];
        for byte in buffer {
            match byte {
                0 => break,
                _ => v.push(byte)
            }
        }
        return String::from_utf8_lossy(&v[..]).to_string();
    }
    else {
        return String::new();
    }
}

