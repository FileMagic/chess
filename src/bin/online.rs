// https://stackoverflow.com/questions/69529963/how-can-i-automate-a-chess-board-with-selenium
// https://dev.to/stevepryde/using-selenium-with-rust-aca
// https://en.wikipedia.org/wiki/Computer_vision

use std::i8;

use thirtyfour::prelude::{ScriptRet, WebDriverError};
use thirtyfour::{WebDriver, DesiredCapabilities, WebElement};
use tokio;
struct Square {
    left: u64,
    right: u64,
    bottom: u64,
    top: u64,
}

impl Square {
    fn width(&self) -> u64 {
        self.width = self.left - self.right
    }
}

async fn click_square(driver: WebDriver, square: Square){
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
    let x_offset = square.left + (square.width/2);
    let y_offset = square.top + int(square.width/2);
    let ac = driver
        .action_chain()
        .move_to_element_center(&element)
        .move_by_offset(x_offset, y_offset).click().perform()
;
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let web_site = "https://www.chess.com/play/computer";
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto(web_site).await?;
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
    ", Vec::new());
    click_square(driver, board[0][0]);
    Ok(())
}
