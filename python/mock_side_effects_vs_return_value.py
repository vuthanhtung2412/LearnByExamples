import unittest.mock


def side_effect_factory(values, default_value):
    def effect():
        for value in values:
            yield value
        while True:
            yield default_value

    return effect()


m = unittest.mock.Mock(
    return_value="foo",
    side_effect=side_effect_factory([1, 2, unittest.mock.DEFAULT, 4, 5], "foo"),
)
for _ in range(6):
    print(m())

print("---")

m1 = unittest.mock.Mock(
    return_value="foo", side_effect=[1, 2, unittest.mock.DEFAULT, 4, 5]
)

for _ in range(5):
    print(m1())

print("---")

print(
    unittest.mock.Mock(return_value="foo", side_effect=lambda: unittest.mock.DEFAULT)()
)
