# Globo Play Rust CLI

A command-line tool for interacting with the Globo Play API, written in Rust. This tool allows users to fetch video information, list videos by date, and download video content.

## Features

*   **Fetch Video Information**: Get basic details or full session information (including stream URLs) for a specific video ID.
*   **List Videos by Date**: Retrieve a list of videos for a given program (title ID) within a specified date range.
*   **Download Videos**: Download video streams.
*   **Authentication**: Supports using a Netscape cookie file for authentication.
*   **Configurable**:
    *   Specify video quality for downloads (low, medium, high, max).
    *   Define output directory for downloads.
    *   Choose output format for information (JSON, pretty JSON, compact).
    *   Enable debug mode for verbose logging.
*   **Cross-platform**: Built with Rust, aiming for compatibility across different operating systems.

## Prerequisites

*   Rust programming language and Cargo (Rust's package manager). Installation instructions can be found at [rust-lang.org](https://www.rust-lang.org/tools/install).
*   (Optional) A valid Netscape format cookie file from an authenticated Globo Play session for accessing restricted content.

## Installation & Setup

1.  **Clone the repository (if applicable) or navigate to the project directory.**
    ```bash
    # git clone <repository_url>
    # cd globo-play-rust
    ```
2.  **Build the project using Cargo:**
    ```bash
    cargo build
    ```
    For a release build (optimized):
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/debug/globo_play_rust` or `target/release/globo_play_rust`.

## Usage

The CLI provides several commands and global options.

**Global Options:**

*   `--cookie <FILE_PATH>` or `-c <FILE_PATH>`: Path to your Netscape cookie file.
    *Example: `--cookie ~/.config/globo-play-cookies.txt`*
*   `--quality <QUALITY>`: Set default video quality for downloads. Options: `low`, `medium`, `high`, `max` (default: `max`).
    *Example: `--quality 720p` (Note: current implementation uses predefined keywords, specific resolution matching might be a future enhancement)*
*   `--output <FORMAT>`: Set output format for information. Options: `json`, `pretty` (default), `compact`.
    *Example: `--output json`*
*   `--debug` or `-d`: Enable debug mode for verbose output.
*   `--output-dir <DIRECTORY>`: Set default directory for downloaded videos (default: current directory `.`).
    *Example: `--output-dir ~/Downloads/GloboPlay`*

**Commands:**

### 1. `video` - Get basic info about a video

Fetches and displays basic information for a given video ID.

```bash
./target/debug/globo_play_rust video <VIDEO_ID> [OPTIONS]
```

**`video` specific options:**

*   `--download`: Download the video.
*   `--filename <FILENAME>`: Custom filename for the downloaded video (extension will be added based on stream type, typically .mp4 or .ts).
*   `--quality <QUALITY>`: Override global video quality for this specific download.
*   `--output-dir <DIRECTORY>`: Override global output directory for this specific download.

**Examples:**

*   Get basic info for video `1234567`:
    ```bash
    ./target/debug/globo_play_rust video 1234567
    ```
*   Get basic info and download video `1234567` with high quality to a custom directory:
    ```bash
    ./target/debug/globo_play_rust video 1234567 --download --quality high --output-dir /path/to/videos
    ```
*   Download video `1234567` using a cookie file:
    ```bash
    ./target/debug/globo_play_rust --cookie cookies.txt video 1234567 --download
    ```

### 2. `video-info` - Get detailed info with sources

Fetches and displays detailed information for a given video ID, including available stream sources.

```bash
./target/debug/globo_play_rust video-info <VIDEO_ID> [OPTIONS]
```

**`video-info` specific options:**

*   `--download`: Download the video.
*   `--filename <FILENAME>`: Custom filename for the downloaded video.
*   `--quality <QUALITY>`: Override global video quality for this specific download.
*   `--output-dir <DIRECTORY>`: Override global output directory for this specific download.

**Examples:**

*   Get detailed info for video `1234567`:
    ```bash
    ./target/debug/globo_play_rust video-info 1234567
    ```
*   Get detailed info and download video `1234567` with max quality:
    ```bash
    ./target/debug/globo_play_rust video-info 1234567 --download --quality max
    ```

### 3. `videos-by-date` - Get videos by date range

Lists videos for a specific program (title ID) within a given date range.

```bash
./target/debug/globo_play_rust videos-by-date <TITLE_ID> [OPTIONS]
```
*(Note: The CLI definition in `cli.rs` for `VideosByDate` seems to be missing `title_id`, `from_date`, and `to_date` arguments. The following examples assume they will be added as per the `handle_videos_by_date_command` function in `main.rs`)*

**Assumed `videos-by-date` arguments (based on `main.rs`):**

*   `<TITLE_ID>`: The ID of the program/show.
*   `[FROM_DATE]`: Start date in YYYY-MM-DD format (optional, defaults to today).
*   `[TO_DATE]`: End date in YYYY-MM-DD format (optional, defaults to `FROM_DATE` or today).

**`videos-by-date` specific options:**

*   `--download-all`: Download all videos fetched by the command.

**Examples (assuming CLI arguments are updated):**

*   List videos for title `program123` for today's date:
    ```bash
    ./target/debug/globo_play_rust videos-by-date program123
    ```
*   List videos for title `program123` from `2023-01-01` to `2023-01-05`:
    ```bash
    ./target/debug/globo_play_rust videos-by-date program123 2023-01-01 2023-01-05
    ```
*   List and download all videos for title `program123` for `2023-02-10`:
    ```bash
    ./target/debug/globo_play_rust videos-by-date program123 2023-02-10 --download-all --cookie cookies.txt
    ```

## Configuration

### Cookie File

For accessing content that requires authentication, you need to provide a cookie file. This file should be in the <abbr title="A Netscape-format cookie file is a plain text file that stores cookies, typically used by browsers or tools like curl. Each line represents a cookie, with tab-separated fields like domain, path, secure, expiration, name, and value. This tool uses it to make authenticated requests to the Globo Play API.">Netscape cookie format</abbr>.

You can typically obtain this by:
1.  Logging into Globo Play in your web browser.
2.  Using a browser extension (e.g., "Get cookies.txt" for Chrome/Firefox) to export the cookies for the `globo.com` domain.
3.  Save the exported content into a plain text file.

Pass the path to this file using the `--cookie` or `-c` global option.

### Debug Mode

To see detailed logs, including API URLs being fetched and full responses (in case of errors or for inspection), use the `--debug` or `-d` global flag.

```bash
./target/debug/globo_play_rust --debug video-info 1234567
```

## Development & TODOs

This project is under development. Potential future enhancements and areas for improvement include:

*   **Refine CLI for `videos-by-date`**: Ensure `title_id`, `from_date`, and `to_date` are correctly implemented as positional or named arguments in `cli.rs`.
*   **Implement `fetch_video_details`**: Uncomment and complete the `fetch_video_details` function in `api.rs` if a separate endpoint for non-session video metadata is useful.
*   **Pagination for `videos-by-date`**: Currently fetches only the first page. Implement logic to handle pagination (`next` URL from `DatedVideosResponse`).
*   **Advanced Quality Selection**: Parse quality labels (e.g., "1080p", "720p") more robustly in `select_best_stream` instead of relying solely on "max"/"min" or API order.
*   **Output Formatting**: Implement the `compact` output format in `utils.rs` for a more user-friendly text representation of data.
*   **Download Progress Bar**: Enhance the `download_file` utility in `utils.rs` with a progress indicator.
*   **Resumable Downloads**: Explore adding support for resumable downloads.
*   **Error Handling**: Continuously improve error messages and handling for API errors and network issues.
*   **Configuration File**: Consider loading default settings (cookie path, quality, output dir) from a TOML configuration file (e.g., `~/.config/globo-play-rust/config.toml`) as hinted in `config.rs`.
*   **Testing**: Add unit and integration tests.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.
