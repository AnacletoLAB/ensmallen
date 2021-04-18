class Modifiers:
    def __init__(self):
        self.mutable = False
        self.reference = False
        self.reference_lifetime = None

    def set_reference(self):
        self.reference = True

    def is_reference(self):
        return self.reference
        
    def set_reference_lifetime(self):
        self.reference_lifetime = True

    def get_reference_lifetime(self):
        return self.reference_lifetime

    def set_mutable(self):
        self.mutable = True

    def is_mutable(self):
        return self.mutable