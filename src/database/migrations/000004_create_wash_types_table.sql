-- Create the WashType table
CREATE TABLE WashType (
    ID SERIAL PRIMARY KEY,
    Name VARCHAR(100) NOT NULL,
    Description TEXT,
    BaseTime INT NOT NULL, -- base time in minutes
    Price DECIMAL(10, 2) NOT NULL -- price in dollars
);