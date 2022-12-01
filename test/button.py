import json
import requests
import socket

from abc import ABCMeta, abstractmethod

class Button(metaclass=ABCMeta):
    @abstractmethod
    def right_click(self):
        ...

    @abstractmethod
    def left_click(self):
        ...

    @abstractmethod
    def both_click(self):
        ...

    @abstractmethod
    def close(self):
        ...

class ButtonTCP(Button):
    def __init__(self, server: str, port: int) -> None:
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.connect((server, port))

    def right_click(self):
        self.socket.sendall(b"Rr")

    def left_click(self):
        self.socket.sendall(b"Ll")

    def both_click(self):
        self.socket.sendall(b"LRlr")

    def close(self):
        self.socket.close()

class ButtonAPI(Button):
    def __init__(self, server: str, port: int):
        self.url = "http://" + server + ":" + str(port)

    def left_click(self):
        requests.post(self.url + "/button/left", data=json.dumps({'action': 'press-and-release'}))

    def right_click(self):
        requests.post(self.url + "/button/right", data=json.dumps({'action': 'press-and-release'}))

    def both_click(self):
        requests.post(self.url + "/button/both", data=json.dumps({'action': 'press-and-release'}))

    def close(self):
        pass
