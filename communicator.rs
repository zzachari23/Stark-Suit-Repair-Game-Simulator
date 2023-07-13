#[derive(Debug)]
#[derive(PartialEq)]


pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands are converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
    pub fn as_str (&self) -> String {

       match self 
       {
           Command::Power(bvalue,number) => 
           {
            let mut s = String::from("Power ");
            let mut num = number.to_string();
            let newnum = num.strip_prefix("-");
            if bvalue == &true {s.push_str("increased by ");}
            if bvalue == &false {s.push_str("decreased by ");}
            match newnum
            {   
                Some(x) => 
                {
                    num = x.to_string();
                    s.push_str(&num);
                },
                None => {s.push_str(&num);},
            }
             s.push_str("%");
             s
           },
           Command::Missiles(bvalue, number) => 
           {
            let mut s = String::from("Missiles ");
            let mut num = number.to_string();
            let newnum = num.strip_prefix("-");
            if bvalue == &true {s.push_str("increased by ");}
            if bvalue == &false {s.push_str("decreased by ");}
            match newnum
            {   
                Some(x) => 
                {
                    num = x.to_string();
                    s.push_str(&num);
                },
                None => {s.push_str(&num);},
            }
             s
           },
           Command::Shield(bvalue) => 
           {
            let mut s = String::from("Shield turned ");
            if bvalue == &true {s.push_str("on");}
            if bvalue == &false {s.push_str("off");}
            s
           },
           Command::Try => "Call attempt failed".to_string(),
           Command::Invalid => "Not a command".to_string(),
       }

    }
}

/**
    Method converts a string to a command. 
    List of the format of the input strings shown below.

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/
pub fn to_command(s: &str) -> Command {
  

    if s.starts_with("power inc ") 
    {
        let numstring = s.strip_prefix("power inc ");
        match numstring
        {
            Some(x) => 
            {
                if x.chars().all(char::is_numeric) == true
                {
                  let num = x.parse::<i32>().unwrap();
                  Command::Power(true,num)
                }
                else
                {
                  Command::Invalid
                }
            },

            None => Command::Invalid,
        }
    }
    else if s.starts_with("power dec ") 
    {
        let numstring = s.strip_prefix("power dec ");
        match numstring
        {
            Some(x) => 
            {
                if x.chars().all(char::is_numeric) == true
                {
                  let num = x.parse::<i32>().unwrap();
                  Command::Power(false,num)
                }
                else
                {
                  Command::Invalid
                }
            },

            None => Command::Invalid,
        }
    }
    else if s.starts_with("add ") 
    {
        let mut newstring = s.strip_suffix(" missiles");
        match newstring
        {
            Some(x) => 
            {
                newstring = x.strip_prefix("add ");
                match newstring
                {
                    Some(x) => 
                    {    
                        if x.chars().all(char::is_numeric) == true
                         {
                            let num = x.parse::<i32>().unwrap();
                            Command::Missiles(true,num)
                         }
                        else
                        {
                             Command::Invalid
                        }

                    },

                    None => Command::Invalid,
                }

            },

            None => Command::Invalid,
        }


    }
    else if s.starts_with("fire ") 
    {

        let mut newstring = s.strip_suffix(" missiles");
        match newstring
        {
            Some(x) => 
            {
                newstring = x.strip_prefix("fire ");
                match newstring
                {
                    Some(x) => 
                    {    
                        if x.chars().all(char::is_numeric) == true
                         {
                            let num = x.parse::<i32>().unwrap();
                            Command::Missiles(false,num)
                         }
                        else
                        {
                             Command::Invalid
                        }

                    },

                    None => Command::Invalid,
                }

            },

            None => Command::Invalid,
        }

    }
    else
    {

        match s
             { 
                "shield on" => Command::Shield(true),
                "shield off" => Command::Shield(false),
                "try calling Miss Potts" => Command::Try,
                 _ => Command::Invalid,
             }
    }
}
