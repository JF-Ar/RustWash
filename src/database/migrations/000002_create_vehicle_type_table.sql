-- Create the VehicleSize table
CREATE TABLE vehicle_size (
    id SERIAL PRIMARY KEY,
    description VARCHAR(50) NOT NULL,
    time_factor DECIMAL(3, 2) NOT NULL -- adjustment factor for the wash time
);