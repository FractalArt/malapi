use std::path::PathBuf;
use structopt::StructOpt;

/// Access the Malshare API.
///
/// To be able to access the API, an API key is needed.
/// It can be provided on the command-line using the `--api-key` or
/// `-k` flags.
///
/// A more convenient way might be to store it in an environment variable
/// as follows:
///
/// `export MALSHARE_API_KEY=<api-key>`
#[derive(Debug, StructOpt)]
#[structopt(name = "malshare-rs")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(long, short = "k", env = "MALSHARE_API_KEY", hide_env_values = true)]
    api_key: String,

    #[structopt(long = "api-limit")]
    api_limit: bool,

    #[structopt(long = "api-remaining")]
    remaining: bool,

    #[structopt(long, short = "d", conflicts_with_all = &["file-info"])]
    download: Option<String>,

    #[structopt(long = "file-info")]
    file_info: Option<String>,

    #[structopt(long, short, parse(from_os_str))]
    output: Option<PathBuf>,

    #[structopt(long = "list-hashes-json")]
    list_24: bool,

    #[structopt(long = "list-hashes-raw")]
    list_24_raw: bool,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    // if limit is requested
    if opt.api_limit {
        match malapi::get_api_call_limit(&opt.api_key).await {
            Err(e) => println!("Error retrieving call limit"),
            Ok(limit) => println!("Daily API call limit: {}", limit),
        }
    }

    // if remaining api calls have been queried
    if opt.remaining {
        match malapi::get_remaining_api_calls(&opt.api_key).await {
            Err(_) => println!("Error retrieving remaining api calls"),
            Ok(remain) => println!("Remaining API calls for today: {}", remain),
        }
    }

    // if a sample should be downloaded
    if let Some(hash) = opt.download {
        println!("Downloading file with hash: {}", hash);
        let _ = malapi::download(&opt.api_key, &hash, opt.output).await;
    }

    // if a list with all the hashes from the last 24 hours is requested in json format
    if opt.list_24 {
        match malapi::get_list(&opt.api_key).await {
            Ok(hashes) => println!("{}", hashes),
            Err(_) => println!("Error getting hashes of the last 24 hours"),
        }
    }

    //  if a list with all the hashes from the last 24 hours is requested in plain text format
    if opt.list_24_raw {
        match malapi::get_list_raw(&opt.api_key).await {
            Ok(hashes) => println!("{}", hashes),
            Err(_) => println!("Error getting hashes of the last 24 hours"),
        }
    }

    // if further information for a given file is requested
    if let Some(hash) = opt.file_info {
        match malapi::list_details(&opt.api_key, &hash).await {
            Ok(details) => println!("{details}"),
            Err(_) => println!("Error while fetching details of file with hash: {hash}"),
        }
    }
}
