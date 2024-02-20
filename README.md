##### Disclaimer 
*This is for educationals purposes, hacking is bad, don't do bad things, because it's not good* !!!

CrabeC2 is a Command and Control framework devellopped in Rust by LeG and Shaadx, this project's objetives is to get a post exploitation server that allow the user to download and execute previously saved payload.

#### GUI - Graphical User Interface
The user interface will be an online website accesible using account, here is a list of endpoint :
* / | to get an overview of the C2 activity
* /init | to create dropper for windows or linux
* /machines  | List information about infected PC
* /payload | List and manage payloads
* /task-event | List events
* /doc        | documentation

### Exploitation step
1) The dropper is installed into the victim machine using differents way like phising 
2) Victim machine execute dropper that will exfiltrate basic information (machine name, mac adresse, ip) about the system in order to create a custom payload
3) Victim machine request to the C2 a custom payload that will be executed to gain control over the machine
4) Enjoy

### Client/Agent communication (Malo)
For the moment the communication is only possible via the terminal and it stall the current program execution.
The output from the command send via `sdtin` is then written to `stdout`.
For the moment the execution flow is:
1) The payload connects to the `/api/open-port` route to get an available port
2) A listener is started for the current machine on the specified port
3) Using stdin you can send command to the infected machine
4) The result is printed on stdout()

You can connect multiple machine but the output and input will stack themself on the terminal.

You can also find the file `rev_shell_handler.rs.back` where I tried to make the implementation
of a correct execution flow using threads.
In this file I tried to make the listener on a specific thread where we get the commands from the
`/shell` route and where the output from the infected machine is saved to a specific filename `shellport.out`.
This output could then be read by the user in the webbrowser and the execution flow of the program won't be stopped.
Unfortunatly I didn't have any time left to finish this implementation.


### Logging/Event/Alert
In order to keep track of infected machine and possible attack campain, we need to create a logging, event, alert system.
#### Logging
Those events will be logged by the C2 to logging file and will be displayed on the main dashboard (all info in <> are variable to add to the log):
* ID = 01,  New infected PC detected - `<time> <IP> <Hostname> <Internal_id>`
* ID = 02, Connection opened from an infected PC - `<time> <IP> <Hostname> <Internal_id>`
* ID = 03, Connection closed from an infected PC - `<time> <IP> <Hostname> <Internal_id>`
* ID = 04, User connected - `<time> <IP> <Username>`
* ID = 05, User disconnected - `<time> <IP> <Username>`
* ID = 06, Stage 1 (Dropper) requested - `<time> <IP> <Hostname> <Internal_id> <dropper_id>`
* ID = 07, Stage 2 (Payload) requested - `<time> <IP> <Hostname> <Internal_id> <payload_id>`
* ID = 08, Payload sucseffuly installed - `<time> <IP> <Hostname> <Internal_id> <payload_id>`
* ID = 09, Scheduled task started - `<time> <task_id>`
* ID = 10, Scheduled task ended - `<time> <task_id> <start_time>`
* ID = 11, External discovery attempt of the C2 - `<time> <IP>`
* ID = 12, Command succes - `<time> <IP> <Hostname> <command>`
* ID = 13, Command fail - `<time> <IP> <Hostname> <command>`
* ID = 14, Exfiltration succes - `<time> <IP> <Hostname>`
* ID = 15, Exfiltration fail - `<time> <IP> <Hostname>`
* ID = 16, Unauthorize connection attempt - `<time> <IP>`
* ID = 17, C2 config changed - `<time> <IP> <diff>`
* ID = 18, Unautorized action from user - `<time> <IP> <Username>`
* ID = 19, C2 started - `<time>`
* ID = 20, C2 stopped - `<time>`
* ID = 21, Payload detected


#### Documentation

On the menu bar in the dashboard you can find a specific documentation that explain the different
fonctionnalities of the C2.

