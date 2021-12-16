#!/usr/bin/env python3

INPUT_FILE = 'input16.txt'

import unittest
from abc import ABC, abstractmethod
from dataclasses import dataclass

DEBUG = True
def debug_print(*args, **kwargs):
    if DEBUG:
        print(*args, **kwargs)

def main():
    if 'INPUT_FILE' not in globals():
        unittest.main()
        return
    packet = Packet.parse_text(get_text(INPUT_FILE))
    versions = packet.get_versions()
    print(sum(versions))

class Packet(ABC):
    @abstractmethod
    def get_versions(self):
        pass

    @abstractmethod
    def get_version(self):
        pass

    @abstractmethod
    def parse(version, bits_iter):
        pass

    @staticmethod
    def parse_text(text):
        return Packet.parse_packet(BitsIter(text_to_bits(text)))

    @staticmethod
    def parse_packet(bits_iter):
        version = get_bits_num(bits_iter, 3)
        type_id = get_bits_num(bits_iter, 3)
        if type_id == 4:
            return LiteralPacket.parse(version, bits_iter)
        else:
            return OperatorPacket.parse(version, bits_iter)

@dataclass
class LiteralPacket(Packet):
    version: int
    value: int

    def get_versions(self):
        yield self.version

    def get_version(self):
        return self.version

    @staticmethod
    def parse(version, bits_iter):
        literal_bits = []
        while True:
            continue_bit = next(bits_iter)
            literal_bits.extend(get_bits(bits_iter, 4))
            if not continue_bit:
                break
        value = bits_to_num(literal_bits)
        return LiteralPacket(version, value)

@dataclass
class OperatorPacket(Packet):
    version: int
    sub_packets: [Packet]

    def get_versions(self):
        yield self.version
        for sub_packet in self.sub_packets:
            yield from sub_packet.get_versions()

    def get_version(self):
        return self.version

    @staticmethod
    def parse(version, bits_iter):
        length_type_id = next(bits_iter)
        if length_type_id:
            return OperatorPacket.parse_num_sub_packets(version, bits_iter)
        else:
            return OperatorPacket.parse_total_length(version, bits_iter)

    @staticmethod
    def parse_total_length(version, bits_iter):
        num_packet_bits = get_bits_num(bits_iter, 15)
        sub_packets = []
        orig_index = bits_iter.index
        while bits_iter.index - orig_index < num_packet_bits:
            sub_packets.append(Packet.parse_packet(bits_iter))
        return OperatorPacket(version, sub_packets)

    @staticmethod
    def parse_num_sub_packets(version, bits_iter):
        num_packets = get_bits_num(bits_iter, 11)
        sub_packets = []
        for _ in range(num_packets):
            sub_packets.append(Packet.parse_packet(bits_iter))
        return OperatorPacket(version, sub_packets)

class BitsIter:
    bits: [bool]
    index: int

    def __init__(self, bits):
        self.bits = bits
        self.index = 0

    def __iter__(self):
        return self

    def __next__(self):
        if self.index < len(self.bits):
            bit = self.bits[self.index]
        else:
            bit = None
        self.index += 1
        return bit

def get_bits_num(bits_iter, num_bits):
    return bits_to_num(get_bits(bits_iter, num_bits))

def get_bits(bits_iter, num_bits):
    return [next(bits_iter) for _ in range(num_bits)]

def bits_to_num(bits):
    result = 0
    for bit in bits:
        result *= 2
        if bit:
            result += 1
    return result

def text_to_bits(text):
    bit_string = hex_to_binary(text)
    return [c == '1' for c in bit_string]

def hex_to_binary(text):
    nums = [int(c, 16) for c in text]
    return ''.join(f'{x:04b}' for x in nums)

def flatten(iters):
    for outer in iters:
        for inner in outer:
            yield inner

def get_text(filename):
    with open(filename) as f:
        return f.read().strip()

class Test(unittest.TestCase):
    def test_parse_text(self):
        text = 'D2FE28'
        binary = '110100101111111000101000'
        self.assertEqual(hex_to_binary(text), binary)

    def test_example_literal(self):
        text = 'D2FE28'
        expected = LiteralPacket(6, 2021)
        self.assertEqual(Packet.parse_text(text), expected)

    def test_operator_length_bits(self):
        text = '38006F45291200'
        expected = OperatorPacket(1, [LiteralPacket(6, 10), LiteralPacket(2, 20)])
        self.assertEqual(Packet.parse_text(text), expected)

    def test_operator_num_packets(self):
        text = 'EE00D40C823060'
        expected = OperatorPacket(7, [LiteralPacket(2, 1), LiteralPacket(4, 2), LiteralPacket(1, 3)])
        self.assertEqual(Packet.parse_text(text), expected)

    def test_examples(self):
        text_to_version_sum = [
            ('8A004A801A8002F478', 16),
            ('620080001611562C8802118E34', 12),
            ('C0015000016115A2E0802F182340', 23),
            ('A0016C880162017C3686B18A3D4780', 31),
        ]
        for text, version_sum in text_to_version_sum:
            packet = Packet.parse_text(text)
            versions = packet.get_versions()
            self.assertEqual(sum(versions), version_sum)

if __name__ == '__main__':
    main()
