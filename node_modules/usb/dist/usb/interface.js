"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Interface = void 0;
var bindings_1 = require("./bindings");
var endpoint_1 = require("./endpoint");
var Interface = /** @class */ (function () {
    function Interface(device, id) {
        this.device = device;
        this.id = id;
        /** Integer alternate setting number. */
        this.altSetting = 0;
        this.refresh();
    }
    Interface.prototype.refresh = function () {
        if (!this.device.configDescriptor) {
            return;
        }
        this.descriptor = this.device.configDescriptor.interfaces[this.id][this.altSetting];
        this.interfaceNumber = this.descriptor.bInterfaceNumber;
        this.endpoints = [];
        var len = this.descriptor.endpoints.length;
        for (var i = 0; i < len; i++) {
            var desc = this.descriptor.endpoints[i];
            var c = (desc.bEndpointAddress & bindings_1.LIBUSB_ENDPOINT_IN) ? endpoint_1.InEndpoint : endpoint_1.OutEndpoint;
            this.endpoints[i] = new c(this.device, desc);
        }
    };
    /**
     * Claims the interface. This method must be called before using any endpoints of this interface.
     *
     * The device must be open to use this method.
     */
    Interface.prototype.claim = function () {
        this.device.__claimInterface(this.id);
    };
    Interface.prototype.release = function (closeEndpointsOrCallback, callback) {
        var _this = this;
        var closeEndpoints = false;
        if (typeof closeEndpointsOrCallback === 'boolean') {
            closeEndpoints = closeEndpointsOrCallback;
        }
        else {
            callback = closeEndpointsOrCallback;
        }
        var next = function () {
            _this.device.__releaseInterface(_this.id, function (error) {
                if (!error) {
                    _this.altSetting = 0;
                    _this.refresh();
                }
                if (callback) {
                    callback.call(_this, error);
                }
            });
        };
        if (!closeEndpoints || this.endpoints.length === 0) {
            next();
        }
        else {
            var n_1 = this.endpoints.length;
            this.endpoints.forEach(function (ep) {
                if (ep.direction === 'in' && ep.pollActive) {
                    ep.once('end', function () {
                        if (--n_1 === 0) {
                            next();
                        }
                    });
                    ep.stopPoll();
                }
                else {
                    if (--n_1 === 0) {
                        next();
                    }
                }
            });
        }
    };
    /**
     * Returns `false` if a kernel driver is not active; `true` if active.
     *
     * The device must be open to use this method.
     */
    Interface.prototype.isKernelDriverActive = function () {
        return this.device.__isKernelDriverActive(this.id);
    };
    /**
     * Detaches the kernel driver from the interface.
     *
     * The device must be open to use this method.
     */
    Interface.prototype.detachKernelDriver = function () {
        return this.device.__detachKernelDriver(this.id);
    };
    /**
     * Re-attaches the kernel driver for the interface.
     *
     * The device must be open to use this method.
     */
    Interface.prototype.attachKernelDriver = function () {
        return this.device.__attachKernelDriver(this.id);
    };
    /**
     * Sets the alternate setting. It updates the `interface.endpoints` array to reflect the endpoints found in the alternate setting.
     *
     * The device must be open to use this method.
     * @param altSetting
     * @param callback
     */
    Interface.prototype.setAltSetting = function (altSetting, callback) {
        var _this = this;
        this.device.__setInterface(this.id, altSetting, function (error) {
            if (!error) {
                _this.altSetting = altSetting;
                _this.refresh();
            }
            if (callback) {
                callback.call(_this, error);
            }
        });
    };
    /**
     * Return the InEndpoint or OutEndpoint with the specified address.
     *
     * The device must be open to use this method.
     * @param addr
     */
    Interface.prototype.endpoint = function (addr) {
        return this.endpoints.find(function (item) { return item.address === addr; });
    };
    return Interface;
}());
exports.Interface = Interface;
//# sourceMappingURL=interface.js.map