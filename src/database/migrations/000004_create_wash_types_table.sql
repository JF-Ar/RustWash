-- Create the WashType table
CREATE TABLE wash_type (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    base_time INT NOT NULL, -- base time in minutes
    price DECIMAL(10, 2) NOT NULL -- price in dollars
);