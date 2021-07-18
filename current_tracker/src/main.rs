use tokio::time::delay_for;
use std::time::Duration;
use rspotify::util::get_token;
use rspotify::oauth2::SpotifyOAuth;
use rspotify::client::Spotify;
use rspotify::oauth2::SpotifyClientCredentials;

async fn tick(spotify: &Spotify) {
    let current_track = spotify.current_playing(None).await;
    let wait_time = match current_track {
        Ok(track) => {
            Duration::from_millis(match track {
                Some(details) => {
                    if details.is_playing {
                        let title = details.item.as_ref().map(|i| i.name.as_str()).unwrap_or("Unknown track");
                        let artist = match details.item.as_ref().map(|i| i.artists.first()) {
                            Some(Some(a)) => a.name.as_str(),
                            Some(None) => "Unknown artist",
                            None => "Unknown artist"
                        };
                        println!("Currently Playing: {} by {}", title, artist);
                        let song_duration = details.item.map(|i| i.duration_ms).unwrap_or(details.progress_ms.unwrap_or(10000) * 2);
                        (song_duration - details.progress_ms.unwrap_or(10000) + 100).into()
                    } else {
                        println!("Not Currently Playing, sleeping for 60 seconds");
                        60000 
                    }
                },
                None => {
                    println!("Not Currently Playing, sleeping for 60 seconds");
                    60000
                }
            })
        }
        Err(e) => {
            println!("Unable to fetch track: {:?}", e);
            Duration::from_secs(62)
        }
    };
    delay_for(wait_time).await;
}

#[tokio::main]
async fn main() {
    let mut oauth = SpotifyOAuth::default()
    .scope("user-read-currently-playing user-read-playback-state")
    .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_creds = SpotifyClientCredentials::default().token_info(token_info).build();
            let spotify = Spotify::default().client_credentials_manager(client_creds).build();
            loop {
                tick(&spotify).await;
            }
        }
        None => println!("Unable to authenticate")
    }
}
