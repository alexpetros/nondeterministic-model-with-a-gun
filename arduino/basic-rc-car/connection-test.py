#!python3
# requires pyserial
import sys
import serial
import glob
import time

def write(x):
    arduino.write(bytes(x, 'ascii'))
    time.sleep(0.05)
    data = arduino.readline()
    return data

# It's going to attempt to connect to the first USB, so make sure there's only one plugged in
port = glob.glob('/dev/tty.usb*')[0]
arduino = serial.Serial(port=port, baudrate=9600, timeout=.1)
print("Connected on port " + port)

print("Waiting to establish connection")
time.sleep(2)
print("Now testing the RC car")
commands = """
f1
r1
b1
l1
r1
f1
l1
b1
s1
f1
"""
write(commands)

print("Connection test complete")
