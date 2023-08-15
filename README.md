# Spacecraft

An open source web3 based solution to get humanity to space
more info here: https://github.com/AthulMuralidhar/spacecraft/wiki/Vision

# Structure

## Ground Control
this is the newtwork part of the project. this code handles the decentralised distributed organisation of the whole project. tokenisatoin and fungibility are also added here
- the landing page for the project
- the infrastructure
- onboarding should be done here as well
- API integrations between Core to the public lives here

## Core
this is the IoT part of the project. this is where the actuators code live, the aim is to have this as a root and put all the software related to actual servo movements here
- the embedded rust core for flashing code into the satellite
- every subproject must be a single small binary
- performance and security are critical here
