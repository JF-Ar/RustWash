-- Create the Vehicle table
CREATE TABLE vehicle (
    id SERIAL PRIMARY KEY,
    plate VARCHAR(20) NOT NULL,
    model VARCHAR(100),
    customer_id INT NOT NULL,
    vehicle_size_id INT NOT NULL,
    CONSTRAINT fk_customer
        FOREIGN KEY(customer_id)
            REFERENCES customer(id),
    CONSTRAINT fk_vehicle_size
        FOREIGN KEY(vehicle_size_id)
            REFERENCES vehicle_size(id)
);