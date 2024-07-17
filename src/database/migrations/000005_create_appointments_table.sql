-- Create the Appointment table
CREATE TABLE appointment (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    time TIME NOT NULL,
    customer_id INT NOT NULL,
    vehicle_id INT NOT NULL,
    wash_type_iD INT NOT NULL,
    CONSTRAINT fk_customer_appointment
        FOREIGN KEY(customer_id)
            REFERENCES customer(id),
    CONSTRAINT fk_vehicle_appointment
        FOREIGN KEY(vehicle_id)
            REFERENCES vehicle(id),
    CONSTRAINT fk_wash_type_appointment
        FOREIGN KEY(wash_type_id)
            REFERENCES wash_type(id)
);