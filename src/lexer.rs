pub enum Tokens<'a>{
    Create,
    List,
    Nick,
    Join,
    Arg(&'a str)
}

pub struct Lexer<'a>{
    command: &'a str,
    command_atoms: Vec<&'a str>,
    pub tokens: Vec<Tokens<'a>>
}

impl<'a> Lexer<'a>{
    pub fn new(command: &'a str) -> Self{
        let command = command.get(1..command.len() - 1).unwrap();
        let mut commands: Vec<&'a str> = command.split_ascii_whitespace().collect();
        Lexer{ command: command, command_atoms: commands, tokens: Vec::new() }
    }

    pub fn scan(&mut self){
        // All command are simple, they have the form COMMAND ARG1 ARG2 ARG3 ...
        // each command has predefined number of arguments.
        let command = *self.command_atoms.get(0).unwrap();
        match command{
            // Create a new room, takes at most three arguments: room name is private password
            // Only the first is required. The second is a bool representing if the room is private or not
            // the final is only required if is_private is set to true
            // :create myNewRoom
            // :create myPrivateRoom true asecretpassword
            "create" => {
                if self.command_atoms.len() == 2{
                    let room_name = *self.command_atoms.get(1).unwrap();
                    self.tokens.push(Tokens::Create);
                    self.tokens.push(Tokens::Arg(room_name))
                }
            }
            // list all (non-private) rooms, takes no arguments
            // :list
            "list" => {

            }

            // The next two feel a bit awkward as they're supposed to be "local" commands
            // Change nick, takes three arguments old name and new name and password in that order. 
            // :nick old_name new_name
            "nick" => {

            }

            // Join a room, takes at most two arguments: room name password
            "join" => {
                if self.command_atoms.len() == 2{
                    let room_name = *self.command_atoms.get(1).unwrap();
                    self.tokens.push(Tokens::Join);
                    self.tokens.push(Tokens::Arg(room_name))
                }

            }

            _ => {

            }
        }
    }
}
