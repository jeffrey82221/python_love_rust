class Operation:
    def __init__(self, op):
        self._op = op
    
    def add(self, a, b):
        return a + b

    def mul(self, a, b):
        return a * b

    def div(self, a, b):
        return a / b

    def sub(self, a , b):
        return a - b

    def run(self, a, b):
        if self._op == 'add':
            return self.add(a, b)
        elif self._op == 'mul':
            return self.mul(a, b)
        elif self._op == 'div':
            return self.div(a, b)
        elif self._op == 'sub':
            return self.sub(a, b)

def run_class(operation: str, a: int, b: int) -> str:
    op = Operation(operation)
    return op.run(a, b)