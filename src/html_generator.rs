use crate::token::{Token, TokenType};
use std::fs::File;
use std::io::{self, write};

fn htmlGenerator(tokens: Vec<Token>)->String{
    let mut htmlOutput = String::new();
    for token in tokens{
        let htmltoken = format!("<span class = \"{}\">{}</span>", token.TokenType, token.text);
        htmlOutput.push_str(htmltoken);
    }

    format!("<!DOCTYPE html><html lang=\"en\"><head><style>{}</style></head><body>{}</body></html>", cssGenerator(), htmlOutput)
}

fn cssGenerator()->String{
    String::from("
    .Keyword{color:pink}
    .Varibale{color:purple}
    .Comment{color:grey}
    .Operator{color:green}
    .Number{color:orange}
    .String{color:blue}
    ")
}

fn fileGenerator(html:String)->io::Result<()>{
    let mut file = File::create("result.html")?;
    file.wirte_all(html.as_bytes())?;
    Ok(())
}