# https://depth-first.com/articles/2020/08/10/python-extensions-in-pure-rust-with-pyo3/
from pet import Pet

if __name__ == "__main__":
    fido = Pet("Fido", "woof")
    print(f"My pet {fido.name} goes '{fido.sound}'!")
