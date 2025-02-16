"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Capability = void 0;
var Capability = /** @class */ (function () {
    function Capability(device, id) {
        this.device = device;
        this.id = id;
        if (!device._bosDescriptor) {
            throw new Error('bosDescriptor not found');
        }
        this.descriptor = device._bosDescriptor.capabilities[this.id];
        this.type = this.descriptor.bDevCapabilityType;
        this.data = this.descriptor.dev_capability_data;
    }
    return Capability;
}());
exports.Capability = Capability;
//# sourceMappingURL=capability.js.map