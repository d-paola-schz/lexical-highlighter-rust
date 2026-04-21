use crate::token::{Token};
use std::fs::File;
use std::io::{self, Write};

pub fn html_generator(tokens: Vec<Token>)->String{
    let mut html_output = String::new();
    for token in tokens{
        let htmltoken = format!("<span class = \"{}\">{}</span>", token.token_type.to_str(), token.text);
        html_output.push_str(&htmltoken);
    }

    format!("<!DOCTYPE html><html lang=\"en\"><head><style>{}</style></head><body>{}</body></html>", css_generator(), html_output)
}

fn css_generator()->String{
    String::from("
    .Keyword{color:pink}
    .Variable{color:purple}
    .Comments{color:grey}
    .Operator{color:green}
    .Number{color:orange}
    .String{color:blue}
    .None{color: black}
    ")
}

pub fn file_generator(html:String)->io::Result<()>{
    let mut file = File::create("result.html")?;
    file.write_all(html.as_bytes())?;
    Ok(())
}