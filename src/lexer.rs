#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Tokens<'a>{
    Create,
    List,
    Nick,
    Join,
    Arg(&'a str)
}

#[derive(Clone)]
pub struct Parsed;
#[derive(Clone)]
pub struct Unparsed;

#[derive(Clone)]
pub struct Lexer<'a, State = Unparsed>{
    // command: &'a str,
    state: std::marker::PhantomData<State>,
    command_atoms: Vec<&'a str>,
    pub tokens: Vec<Tokens<'a>>
}

impl<'a> Lexer<'a, Unparsed>{
    pub fn new(command: &'a str) -> Self{
        let command = command.get(1..command.len() - 1).unwrap();
        let commands: Vec<&'a str> = command.split_ascii_whitespace().collect();
        Lexer{ command_atoms: commands, tokens: Vec::new(), state: std::marker::PhantomData::<Unparsed> }
    }

    pub fn scan(&mut self) -> Lexer<Parsed>{
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
                    self.tokens.push(Tokens::Arg(room_name));
                }
            }
            // list all (non-private) rooms, takes no arguments
            // :list
            "list" => {
                self.tokens.push(Tokens::List)
            }

            // The next two feel a bit awkward as they're supposed to be "local" commands
            // Change nick, takes three arguments old name and new name and password in that order. 
            // :nick new_name
            "nick" => {
                if self.command_atoms.len() == 2{
                    let new_nick = *self.command_atoms.get(1).unwrap();
                    self.tokens.push(Tokens::Nick);
                    self.tokens.push(Tokens::Arg(new_nick))
                }

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
        // Can I remove this clone?
        return Lexer::new_parsed(command, self.tokens.clone())
    }
}

impl<'a> Lexer<'a, Parsed>{
    fn new_parsed(command: &'a str, tokens: Vec<Tokens<'a>>) -> Self{
        let command = command.get(1..command.len() - 1).unwrap();
        let commands: Vec<&'a str> = command.split_ascii_whitespace().collect();
        Lexer{ command_atoms: commands, tokens: tokens, state: std::marker::PhantomData::<Parsed> }
    }

    pub fn is_local_command(&self) -> bool{
        return self.tokens.get(0).unwrap() == &Tokens::Join || self.tokens.get(0).unwrap() == &Tokens::Nick
    }
}
