-- Create the VehicleSize table
CREATE TABLE CarsSize (
    ID SERIAL PRIMARY KEY,
    Description VARCHAR(50) NOT NULL,
    TimeFactor DECIMAL(3, 2) NOT NULL -- adjustment factor for the wash time
);