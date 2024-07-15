-- Create the Vehicle table
CREATE TABLE Vehicle (
    ID SERIAL PRIMARY KEY,
    Plate VARCHAR(20) NOT NULL,
    Model VARCHAR(100),
    CustomerID INT NOT NULL,
    VehicleSizeID INT NOT NULL,
    CONSTRAINT fk_customer
        FOREIGN KEY(CustomerID)
            REFERENCES Customer(ID),
    CONSTRAINT fk_vehicle_size
        FOREIGN KEY(VehicleSizeID)
            REFERENCES VehicleSize(ID)
);