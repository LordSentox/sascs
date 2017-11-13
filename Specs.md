# Specifications and usage ideas for the synchronisation

### The push method, default behavior

- The directory it is called in is the one to be synchronised.
- The directory it is synchronised to is the default directory set in the daemon on the remote machine.
- If a file would be overwritten, the most recently edited is always chosen.

#### Warnings
Enable warnings to ask before a file would be overwritten by sinchronising.

#### IP-Address
Push to a specific IP-Address or addresses only.

#### Directory
The directory to be synchronised. This does not affect the root of the directory that is chosen on the remote machine.

#### Remote-Directory
The remote home directory cannot be changed out of security reasons. Also, `../` directory changes are ignored.

#### Password
For starters, there will be no password support. It is encouraged to only use this in the local network with local IPs.

### The sync method
- Checks the system time with all computers running the program in the network, so that the last edited time may be determined with accuracy.
- Does NOT change any systems individual time.
- The reference time is always the system time with the lowest time, all others will be a positive difference.
