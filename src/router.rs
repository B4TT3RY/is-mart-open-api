use tide::Request;
use urlencoding::decode;

pub async fn search(req: Request<()>) -> tide::Result<String> {
    let mart = req.param("mart")?;
    let keyword = decode(req.param("keyword")?)?;
    println!("{}", keyword);
    Ok(format!("mart: {}, keyword: {}\n", mart, keyword))
}