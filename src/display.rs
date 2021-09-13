use dns_lookup::lookup_host;
use std::io::{Read, Write};
use std::net::TcpStream;
use url::Url;
use vgtk::{Component, gtk, UpdateAction, VNode};
use vgtk::ext::*;
use vgtk::lib::gtk::*;

use crate::*;

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

#[derive(Clone, Debug)]
struct HuginnComponent {
    pub label: String
}

impl HuginnComponent {
    fn new<S: ToString>(string: S) -> Self {
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

fn get_arg(text: &str) -> &str {
    let split_right = text.split("@head(").collect::<Vec<&str>>()[1];
    let split_left = split_right.split(")").collect::<Vec<&str>>()[0];
    return split_left;
}

enum List {
    BLIST,
    NLIST,
    NONE
}

fn odin_to_components(text: String) -> Vec<HuginnComponent> {
    let mut label_strings = vec![];
    let mut list_status = List::NONE;
    let mut nlist_index = 1;

    let mut lines = text.split("\n").collect::<Vec<&str>>();
    lines.retain(|line| !line.is_empty());
    for mut line in lines {
        let mut blist_bullet = "  • ".to_string();
        let mut nlist_bullet = format!("  {}. ", nlist_index).to_string();
        match list_status {
            List::NONE => {},
            _ => {
                let split = line.split("*").collect::<Vec<&str>>()[0];
                line = split.trim_start();
            }
        }
        if line.starts_with("@head") {
            let text = "<span size=\"xx-large\"><b>".to_owned() + get_arg(line).trim_end() + "</b></span>";
            label_strings.push(text);
        }
        else if line.starts_with("@hhead") {
            let text = "<span size=\"x-large\"><b>".to_owned() + get_arg(line).trim_end() + "</b></span>";
            label_strings.push(text);
        }
        else if line.starts_with("@hhhead") {
            let text = "<span size=\"large\"><b>".to_owned() + get_arg(line).trim_end() + "</b></span>";
            label_strings.push(text);
        }
        else if line.starts_with("@nlist") {
            match list_status {
                List::NLIST => {
                    nlist_index = 1;
                    list_status = List::NONE;
                },
                _ => list_status = List::NLIST
            }
        }
        else if line.starts_with("@blist") {
            list_status = match list_status {
                List::BLIST => List::NONE,
                _ => List::BLIST
            }
        }
        else {
            match list_status {
                List::BLIST => {
                    blist_bullet.push_str(line);
                    line = &blist_bullet;
                },
                List::NLIST => {
                    nlist_bullet.push_str(line);
                    line = &nlist_bullet;
                    nlist_index += 1;
                },
                _ => {}
            }
            label_strings.push(line.trim_end().to_string());
        }
    }
    return label_strings.iter().map(HuginnComponent::new).collect::<Vec<HuginnComponent>>();
}

