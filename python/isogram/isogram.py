

def is_isogram(string: str):
    isogram = set(string.lower().replace(" ", "").replace("-", ""))
    if len(isogram) + string.count(" ") + string.count("-") == len(string):
        return True
    return False


print(is_isogram("thumbscrew-japingly"))
