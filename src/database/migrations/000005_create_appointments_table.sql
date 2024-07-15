-- Create the Appointment table
CREATE TABLE Appointment (
    ID SERIAL PRIMARY KEY,
    Date DATE NOT NULL,
    Time TIME NOT NULL,
    CustomerID INT NOT NULL,
    VehicleID INT NOT NULL,
    WashTypeID INT NOT NULL,
    CONSTRAINT fk_customer_appointment
        FOREIGN KEY(CustomerID) 
            REFERENCES Customer(ID),
    CONSTRAINT fk_vehicle_appointment
        FOREIGN KEY(VehicleID) 
            REFERENCES Vehicle(ID),
    CONSTRAINT fk_wash_type_appointment
        FOREIGN KEY(WashTypeID) 
            REFERENCES WashType(ID)
);