
# Plugin (CLIENT)

### ServerRegistration
  - **Register** \
    Indicates to the governor that a client would like to register within memory.
    - Id: 0
  - **Response** \
    Indicates to the client that the governor either accepted or denied the registration.
    - Id: 1

### **DisconnectServer**
Notifies the remote client that a forceful disconnection occurred on the governor side. \
TcpStream connection will always be closed after calling this packet
  - Id: 2

### **Ready**
Indicates that the governor has successfully found a node for client-node communications. 
  - Id: 3

### **ProxyMessage**
Proto message within a message intended to be sent to the client corresponding node.
  - Id: 10

# Anticheat (NODE)

### NodeRegistration
  - **Register**
    Indicates to the governor that a node would like to register within memory.
    - Id: 0
  - **Response**
    Indicates to the node that the governor either accepted or denied the registration
    - Id: 1

### **DisconnectNode**
Notifies the remote node that a forceful disconnection occurred on the governor side. \
TcpStream connection will always be closed after calling this packet
  - Id: 2

### **NodeProxyNegotiation**
Notifies the node that it should connect to a reverse proxy connection provided by the governor
  - Id: 3

### **ProxyMessage**
Proto message within a message intended to be sent to the node corresponding client.
  - Id: 10

# Game <-> Anticheat (Proxy)
All packets within this hierarchy should be prefixed with ``Px_`` to denote proxied state skipping governor checks entirely.

### **DisconnectServer**
Notifies the remote node that a disconnection occurred on the governor side.
- Id: 0

### PlayerJoin
New player or already connected player needs to be registered anticheat side.
- Id: 1

### PlayerLeave
Player has disconnected and needs to be removed anticheat side.
- Id: 2

### PlayerMovement
Player has moved position.
- Id: 3

### PlayerRotation
Player performed rotations.
- Id: 4

### PlayerGround
Player's ground status has changed.
- Id: 5