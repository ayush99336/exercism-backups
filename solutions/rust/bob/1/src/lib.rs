pub fn reply(message: &str) -> &str {
    let up=message.to_uppercase();
    let words=vec!(message);
    let contains_letters=message.chars().any(|c| c.is_alphabetic());
    
    let mut is_yelling=true;
    for word in words{
        if word.to_uppercase()!=word{
            is_yelling=false;
            break;
        }
    }
    
    if message.trim().is_empty(){
        return "Fine. Be that way!";
    }

    else if message.trim_end().ends_with("?"){
        if contains_letters==false{
            return "Sure.";
        }
        else if is_yelling{
            return "Calm down, I know what I'm doing!";
        }
        return "Sure.";
    }
        else if is_yelling&&contains_letters{

        return "Whoa, chill out!";
    }
    else if contains_letters==false{
        return "Whatever.";
    }
    return "Whatever.";   
    
}