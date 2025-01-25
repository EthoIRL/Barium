
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