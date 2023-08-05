use serde::{Serialize, Deserialize};

fn main() {

    let mut coin: String = String::new();
    println!("Que criptomoneda quieres consultar??");
    let _ = std::io::stdin().read_line(&mut coin).expect("Ocurrio un error");


    let result_precio = get_precio(&coin);
    match result_precio {
        Ok(precio) => println!("El precio es: \n{}", precio),
        Err(error) => println!("error al buscar precio {}", error)
    }
    
}


fn get_precio(_coin: &str)  -> Result<String, ureq::Error>{
    // request para el precio real de lamoneda criptografica virtual 
    let body: String = ureq::get(&format!("https://api.coingecko.com/api/v3/coins/{}?localization=false", _coin))
    .call()?
    .into_string()?;
    let coin_data: CoinData = serde_json::from_str(&body).unwrap();
    let usd = coin_data.market_data.current_price.usd.to_string();
    let clp = coin_data.market_data.current_price.clp.to_string();
    let resultado = format!("${} -> clp \n${} -> usd", clp, usd);

    Ok(resultado)

}

#[derive(Serialize, Deserialize, Debug)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    market_data: MaerketData,
}
#[derive(Serialize, Deserialize, Debug)]
struct  MaerketData{
    current_price: Prices,
}
#[derive(Serialize, Deserialize, Debug)]
struct Prices{
    usd: f32,
    clp: f32,
}