pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let mut count = 0;

    // Iterate through the characters of the input string
    for (i, c) in source.chars().enumerate() {
        count += 1;

        // If the current character is different from the next one, or it's the last character
        if i + 1 >= source.len() || source.chars().nth(i + 1) != Some(c) {
            if count > 1 {
                // Append the count if it's greater than 1
                encoded.push_str(&count.to_string());
            }
            // Append the character
            encoded.push(c);
            // Reset the count
            count = 0;
        }
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count_str = String::new();

    // Iterate through the characters of the input string
    for c in source.chars() {
        if c.is_numeric() {
            // If the character is numeric, accumulate it to build the count string
            count_str.push(c);
        } else {
            // If the character is not numeric
            if !count_str.is_empty() {
                // If there's a count string, parse it
                let count: usize = count_str.parse().unwrap();
                // Repeat the character count times and append to the decoded string
                decoded.push_str(&c.to_string().repeat(count));
                // Reset the count string
                count_str.clear();
            } else {
                // If there's no count string, simply append the character to the decoded string
                decoded.push(c);
            }
        }
    }

    decoded
}
