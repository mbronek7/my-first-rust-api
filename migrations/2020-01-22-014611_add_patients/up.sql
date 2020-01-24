CREATE TABLE doctors (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  specialization VARCHAR NOT NULL
);

CREATE TABLE patients (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  second_name VARCHAR NOT NULL,
  phone_number VARCHAR NOT NULL,
  email VARCHAR NOT NULL
);

CREATE TABLE visits (
  id SERIAL PRIMARY KEY,
  visit_name VARCHAR NOT NULL,
  visit_date VARCHAR NOT NULL,
  doctor_id INT NOT NULL,
  patient_id INT NOT NULL,
  FOREIGN KEY (doctor_id) REFERENCES doctors(id),
  FOREIGN KEY (patient_id) REFERENCES patients(id)
);

INSERT INTO patients(id, first_name, second_name, phone_number, email) VALUES (1, 'Frank', 'Dollar', '123123123', 'frank@dollar.com');
INSERT INTO doctors(id, name, specialization) VALUES (1, 'John Doe', 'cardiologist');
INSERT INTO visits(id, visit_name, visit_date, doctor_id, patient_id) VALUES (1, 'test visit', '12.02.2020', 1, 1);
