
pub fn get_art_lines(art: String) -> Vec<String> {
    art.lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn reallign_text(text: String, max_width: usize) -> String {
    // AI do this!
    let mut result = String::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if word.len() > max_width {
            // if line is being written, print it
            if !current_line.is_empty() {
                result.push_str(&current_line);
                result.push('\n');
                current_line.clear();
            }

            // split long words into parts
            for chunk in word.as_bytes().chunks(max_width) {
                let segment = String::from_utf8_lossy(chunk);
                result.push_str(&segment);
                result.push('\n');
            }
        } else {
            // continous merging if not excceed max_width
            if current_line.len() + word.len() + if current_line.is_empty() { 0 } else { 1 } <= max_width {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            } else {
                // push down old line
                result.push_str(&current_line);
                result.push('\n');
                current_line = word.to_string();
            }
        }
    }

    // push down last line
    if !current_line.is_empty() {
        result.push_str(&current_line);
        result.push('\n');
    }

    result
}

pub trait ArtDimension {
    fn height(&self) -> usize;
    fn width(&self) -> usize;
}

impl ArtDimension for String {
    fn height(&self) -> usize {
        get_art_lines(self.clone()).len()
    }

    fn width(&self) -> usize {
        get_art_lines(self.clone())
            .iter()
            .map(|s| { s.len() })
            .max()
            .unwrap()
    }
}

pub const MENU_INSTRUCTION: &'static str = concat!(
"              Instructions              \n",
"========================================\n",
"  mkroom [3|4|5] : create a new room    \n",
"  cdroom [rid] : join an existing room  \n",
"  exit : exit the application           \n"
);

pub const PROMPT_BOX: &'static str = concat!(
"                PROMPT BOX              \n",
"========================================\n",
" >                                      \n",
"========================================\n",
"                                        \n",
"                                        \n"
);

pub const HELLO: &'static str = "Helllo, turning out that i do program this for pain, pretty much!";
pub const TEST: &'static str = "jshdsdjncxknjcsdbfjsnncjsbcjdnnvjsdsndjbscjdjsdnvsfbsjdsjsdn";

pub const BACKGROUND_404: &'static str = concat!(
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                 ###########                                 ############                               ###########                    \n",
"                ###########                             ######################                         ###########                     \n",
"               ###########                          ###############################                   ###########                      \n",
"              ###########                       ######################################               ###########                       \n",
"             ###########                       ############                ############             ###########                        \n",
"            ###########                        ##########                    ##########            ###########                         \n",
"           ###########                         ##########                    ##########           ###########                          \n",
"          ###########                          ##########                    ##########          ###########                           \n",
"         ###########                           ##########                    ##########         ###########                            \n",
"        ###########     ##########             ##########                    ##########        ###########     ##########              \n",
"       ###########      ##########             ##########                    ##########       ###########      ##########              \n",
"      ###########       ##########             ##########                    ##########      ###########       ##########              \n",
"     ###########        ##########             ##########                    ##########     ###########        ##########              \n",
"    #######################################    ##########                    ##########    #######################################     \n",
"    #######################################    ##########                    ##########    #######################################     \n",
"    #######################################    ##########                    ##########    #######################################     \n",
"    #######################################    ############                ############    #######################################     \n",
"                        ##########              ######################################                         ##########              \n",
"                        ##########                 ###############################                             ##########              \n",
"                        ##########                      ######################                                 ##########              \n",
"                        ##########                           ############                                      ##########              \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n",
"                                                                                                                                       \n"
);