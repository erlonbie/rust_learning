use prettytable::{color, format, row, table, Attr, Cell, Row, Table};
use reqwest::*;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<()> {

    //------------------------------------------------------------------//
    //                          Data Gathering                          //
    //------------------------------------------------------------------//

    //source url adress
    let url = "https://www.terra.com.br/esportes/futebol/brasileiro-serie-a/tabela/";
    //get the html from the url
    let response = reqwest::get(url).await?.text().await?;

    //parse the html
    let document = Html::parse_document(&response);
    //get the table rows
    let selector = Selector::parse("table tr td").unwrap();
    //get the table data
    let td_elements = document.select(&selector);

    let mut items = Vec::new();

    //------------------------------------------------------------------//
    //                          Data Cleaning                           //
    //------------------------------------------------------------------//

    //iterate over the table data and push the inner html to the items vector
    for td in td_elements {
        if let Some(a_element) = td.select(&Selector::parse("a").unwrap()).next() {
            items.push(a_element.inner_html());
        } else {
            items.push(td.inner_html());
        };
    }

    //split the items vector into a vector of vectors with 13 elements each
    let mut new_items = items
        .chunks(13)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();
    for inner_vec in new_items.iter_mut() {
        inner_vec.remove(1);
    }

    //remove the span tags from the third of each inner vector
    let transformed_vec: Vec<Vec<_>> = new_items
        .into_iter()
        .map(|mut inner_vec| {
            if let Some(second_element) = inner_vec.get_mut(2) {
                *second_element = second_element.replace("<span>", "").replace("</span>", " ");
            }
            inner_vec
        })
        .collect();

    //remove the span tags from the second element of each inner vector
    let transformed_vec: Vec<Vec<String>> = transformed_vec
        .iter()
        .map(|inner_vec| {
            inner_vec
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let item_str = item.to_string();
                    let modified_item = if index == 1 {
                        let span_index = item_str.find("<span>").unwrap_or(item_str.len());
                        let word = &item_str[..span_index];
                        word.trim().to_string()
                    } else {
                        item_str
                    };
                    modified_item
                })
                .collect()
        })
        .collect();

    //------------------------------------------------------------------//
    //                       Data transformation                        //
    //------------------------------------------------------------------//

    //convert the inner vectors to vectors of string slices
    let transformed_data: Vec<Vec<&str>> = transformed_vec
        .iter()
        .map(|inner_vec| inner_vec.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
        .collect();

    //create the table
    let mut table = Table::new();
    //set the table format
    table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
    //add the table headers
    table.add_row(
        row![bFy => "Pos", "Time", "Mudanca", "P", "J", "V", "E", "D", "GP", "GC", "SG", "Aproveitamento(%)"],
    );

    //add the table rows
    for row in transformed_data {
        let mut table_row = Row::empty();
        //for each row, check some conditions to aply some format rules
        for (i, col) in row.iter().enumerate() {
            let cell = match i {
                //if the first column is a number and less than 5, apply the color blue
                0 if col.parse::<i32>().is_ok() && col.parse::<i32>().unwrap() < 5 => {
                    Cell::new(col).with_style(Attr::ForegroundColor(color::BLUE))
                }
                //if the first column is a number and less than 7, apply the color magenta
                0 if col.parse::<i32>().is_ok() && col.parse::<i32>().unwrap() < 7 => {
                    Cell::new(col).with_style(Attr::ForegroundColor(color::MAGENTA))
                }
                //if the first column is a number and less than 13, apply the color green
                0 if col.parse::<i32>().is_ok() && col.parse::<i32>().unwrap() < 13 => {
                    Cell::new(col).with_style(Attr::ForegroundColor(color::GREEN))
                }
                //if the first column is a number and greater than 16, apply the color red
                0 if col.parse::<i32>().is_ok() && col.parse::<i32>().unwrap() >= 17 => {
                    Cell::new(col).with_style(Attr::ForegroundColor(color::RED))
                }
                //if the third column contains the word "Subiu", apply the color green
                2 if col.contains("Subiu") => {
                    Cell::new(col).with_style(Attr::ForegroundColor(color::GREEN))
                }
                //if the third column contains the word "Desceu", apply the color red
                2 if col.contains("Desceu") => {
                    Cell::new(col).with_style(Attr::ForegroundColor(color::RED))
                }
                //if the tird colum is empty, add the string "----" with the color bright black
                2 if col.is_empty()  => {
                    Cell::new("----").with_style(Attr::ForegroundColor(color::BRIGHT_BLACK))
                }
                //if the 10th column is a number and greater than 0, apply the color green
                10 if col.parse::<i32>().unwrap() > 0 => {
                    Cell::new(col).with_style(Attr::ForegroundColor(color::BRIGHT_GREEN))
                }
                //if the 10th column is a number and less than 0, apply the color red
                10 if col.parse::<i32>().unwrap() < 0 => {
                    Cell::new(col).with_style(Attr::ForegroundColor(color::BRIGHT_RED))
                }
                //else, just add the cell
                _ => Cell::new(col),
            };
            table_row.add_cell(cell);
        }
        table.add_row(table_row);
    }

    //------------------------------------------------------------------//
    //                           Data Output                            //
    //------------------------------------------------------------------//

    //print the table
    table.printstd();

    //Table information
    let mut table2 = table!(["Coluna", "Descrição"],
                        ["P", "Pontos"],
                        ["J", "Jogos"],
                        ["V", "Vitórias"],
                        ["E", "Empates"],
                        ["D", "Derrotas"],
                        ["GP", "Gols pro"],
                        ["GC", "Gols contra"],
                        ["SG", "Saldo de gols"]);
    table2.set_format(*format::consts::FORMAT_BORDERS_ONLY);
    table2.printstd();


    Ok(())
}
