// https://stackoverflow.com/questions/69529963/how-can-i-automate-a-chess-board-with-selenium
// https://dev.to/stevepryde/using-selenium-with-rust-aca
// https://en.wikipedia.org/wiki/Computer_vision

use serde::{Serialize, Deserialize};
use thirtyfour::{WebDriver, DesiredCapabilities, WebElement};
use std::collections::HashMap;
use tokio;

#[derive(Serialize, Deserialize, Debug)]
struct Square {
    piece: String,
    height: u16,
    left: u16,
    top: u16,
    width: u16,
}

impl Square {
    fn center() {

    }
}

async fn click_square(driver: &WebDriver, square: Square){
    let script_return = driver.execute(
            "return document.querySelector(\'body\')", 
            Vec::new()
        ).await;
    let body = match script_return {
        Ok(body) => body,
        Err(error) => panic!("Could not get the body from the webpage. {:?}", error),
    };
    let element: WebElement = match body.element() {
        Ok(element) => element,
        Err(error) => panic!("Problem getting element from body {:?}", error),
    };
    let x_offset: i64 = square.left as i64 + (square.width/2) as i64;
    let y_offset: i64 = square.top as i64 + (square.width/2) as i64 ;
    
    println!("The center of the square is x: {x_offset}, y: {y_offset}");
    let action_chain = driver
        .action_chain()
        .move_to_element_center(&element)
        .move_by_offset(x_offset, y_offset).click().perform();
    let chain_result = match action_chain.await {
        Ok(chain_result) => chain_result,
        Err(error) => {
            match driver.to_owned().quit().await {
                Ok(it) => it,
                Err(err) => panic!("Error while killing driver during move error: {err}"),
            };
            panic!("An error has occurred while pulling out the click action chain: {error}");
        }
    };
    chain_result
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    
    // 
    let piece_names = HashMap::from([
        ("wp".to_string(), "white pawn".to_string()),
        ("bp".to_string(), "black pawn".to_string()),
        ("wb".to_string(), "white bishop".to_string()),
        ("bb".to_string(), "black bishop".to_string()),
        ("wk".to_string(), "white knight".to_string()),
        ("bk".to_string(), "black knight".to_string()),
        ("wr".to_string(), "white rook".to_string()),
        ("br".to_string(), "black rook".to_string()),
        ("wq".to_string(), "white queen".to_string()),
        ("bq".to_string(), "black queen".to_string()),
        ("wn".to_string(), "white king".to_string()),
        ("bn".to_string(), "black king".to_string()),
    ]);
    let web_site = "https://www.chess.com/play/computer";
    let caps = DesiredCapabilities::firefox();
    let selenium_server_location = "http://localhost:4444";
    
    println!("Selenium server location: \"{selenium_server_location}\".");
    let driver = WebDriver::new(selenium_server_location, caps).await?;
    
    println!("Going to website \"{web_site}\".");
    driver.goto(web_site).await?;
    
    println!("Running Script.");
    let board = driver.execute("
        function coords(elem){
            var n = elem.getBoundingClientRect()
            return {top:n.top, left:n.left, width:n.width, height:n.height}
        }
        var pieces = []
        for (var i = 1; i < 9; i++){
            if (i > 6 || i < 3){
                pieces.push(Array.from((new Array(8)).keys()).map(function(x){
                var square = document.querySelector(`.piece.square-${x+1}${i}`)
                return {...coords(square), piece:square.getAttribute('class').split(' ')[1]}
                }));
            }
            else{
                pieces.push(Array.from((new Array(8)).keys()).map(function(x){
                var arr = pieces[pieces.length-1]
                return {left:arr[x].left, top:arr[x].top - arr[x].height, 
                    width:arr[x].width, height:arr[x].height, piece:null}
                }));
            }
        }
        return pieces
    ", Vec::new()).await?;

    println!("Printing JSON response.");
    println!("{}", board.json());
    let piece = Square{piece: "br".to_string(), height:81,left:0, top:0, width:81};
    let click_response = click_square(&driver, piece).await;
    println!("{:?}", click_response);

    println!("Closing the browser!");
    // Always explicitly close the browser.
    driver.quit().await?;
    Ok(())
}
