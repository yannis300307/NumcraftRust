"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.OutEndpoint = exports.InEndpoint = exports.Endpoint = void 0;
var events_1 = require("events");
var bindings_1 = require("./bindings");
var isBuffer = function (obj) { return obj && obj instanceof Uint8Array; };
/** Common base for InEndpoint and OutEndpoint. */
var Endpoint = /** @class */ (function (_super) {
    __extends(Endpoint, _super);
    function Endpoint(device, descriptor) {
        var _this = _super.call(this) || this;
        _this.device = device;
        /** Sets the timeout in milliseconds for transfers on this endpoint. The default, `0`, is infinite timeout. */
        _this.timeout = 0;
        _this.descriptor = descriptor;
        _this.address = descriptor.bEndpointAddress;
        _this.transferType = descriptor.bmAttributes & 0x03;
        return _this;
    }
    /** Clear the halt/stall condition for this endpoint. */
    Endpoint.prototype.clearHalt = function (callback) {
        return this.device.__clearHalt(this.address, callback);
    };
    /**
     * Create a new `Transfer` object for this endpoint.
     *
     * The passed callback will be called when the transfer is submitted and finishes. Its arguments are the error (if any), the submitted buffer, and the amount of data actually written (for
     * OUT transfers) or read (for IN transfers).
     *
     * @param timeout Timeout for the transfer (0 means unlimited).
     * @param callback Transfer completion callback.
     */
    Endpoint.prototype.makeTransfer = function (timeout, callback) {
        return new bindings_1.Transfer(this.device, this.address, this.transferType, timeout, callback);
    };
    return Endpoint;
}(events_1.EventEmitter));
exports.Endpoint = Endpoint;
/** Endpoints in the IN direction (device->PC) have this type. */
var InEndpoint = /** @class */ (function (_super) {
    __extends(InEndpoint, _super);
    function InEndpoint(device, descriptor) {
        var _this = _super.call(this, device, descriptor) || this;
        /** Endpoint direction. */
        _this.direction = 'in';
        _this.pollTransfers = [];
        _this.pollTransferSize = 0;
        _this.pollPending = 0;
        _this.pollActive = false;
        return _this;
    }
    /**
     * Perform a transfer to read data from the endpoint.
     *
     * If length is greater than maxPacketSize, libusb will automatically split the transfer in multiple packets, and you will receive one callback with all data once all packets are complete.
     *
     * `this` in the callback is the InEndpoint object.
     *
     * The device must be open to use this method.
     * @param length
     * @param callback
     */
    InEndpoint.prototype.transfer = function (length, callback) {
        var _this = this;
        var buffer = Buffer.alloc(length);
        var cb = function (error, _buffer, actualLength) {
            callback.call(_this, error, buffer.slice(0, actualLength));
        };
        try {
            this.makeTransfer(this.timeout, cb).submit(buffer);
        }
        catch (e) {
            process.nextTick(function () { return callback.call(_this, e); });
        }
        return this;
    };
    /**
     * Start polling the endpoint.
     *
     * The library will keep `nTransfers` transfers of size `transferSize` pending in the kernel at all times to ensure continuous data flow.
     * This is handled by the libusb event thread, so it continues even if the Node v8 thread is busy. The `data` and `error` events are emitted as transfers complete.
     *
     * The device must be open to use this method.
     * @param nTransfers
     * @param transferSize
     */
    InEndpoint.prototype.startPoll = function (nTransfers, transferSize, _callback) {
        var _this = this;
        var transferDone = function (error, transfer, buffer, actualLength) {
            if (!error) {
                _this.emit('data', buffer.slice(0, actualLength));
            }
            else if (error.errno != bindings_1.LIBUSB_TRANSFER_CANCELLED) {
                _this.emit('error', error);
                _this.stopPoll();
            }
            if (_this.pollActive) {
                startTransfer(transfer);
            }
            else {
                _this.pollPending--;
                if (_this.pollPending === 0) {
                    _this.pollTransfers = [];
                    _this.pollActive = false;
                    _this.emit('end');
                }
            }
        };
        var startTransfer = function (transfer) {
            try {
                transfer.submit(Buffer.alloc(_this.pollTransferSize), function (error, buffer, actualLength) {
                    transferDone(error, transfer, buffer, actualLength);
                });
            }
            catch (e) {
                _this.emit('error', e);
                _this.stopPoll();
            }
        };
        this.pollTransfers = this.startPollTransfers(nTransfers, transferSize, function (error, buffer, actualLength) {
            transferDone(error, this, buffer, actualLength);
        });
        this.pollTransfers.forEach(startTransfer);
        this.pollPending = this.pollTransfers.length;
        return this.pollTransfers;
    };
    InEndpoint.prototype.startPollTransfers = function (nTransfers, transferSize, callback) {
        if (nTransfers === void 0) { nTransfers = 3; }
        if (transferSize === void 0) { transferSize = this.descriptor.wMaxPacketSize; }
        if (this.pollActive) {
            throw new Error('Polling already active');
        }
        this.pollTransferSize = transferSize;
        this.pollActive = true;
        this.pollPending = 0;
        var transfers = [];
        for (var i = 0; i < nTransfers; i++) {
            var transfer = this.makeTransfer(0, callback);
            transfers[i] = transfer;
        }
        return transfers;
    };
    /**
     * Stop polling.
     *
     * Further data may still be received. The `end` event is emitted and the callback is called once all transfers have completed or canceled.
     *
     * The device must be open to use this method.
     * @param callback
     */
    InEndpoint.prototype.stopPoll = function (callback) {
        if (!this.pollActive) {
            throw new Error('Polling is not active.');
        }
        for (var i = 0; i < this.pollTransfers.length; i++) {
            try {
                this.pollTransfers[i].cancel();
            }
            catch (error) {
                this.emit('error', error);
            }
        }
        this.pollActive = false;
        if (callback)
            this.once('end', callback);
    };
    return InEndpoint;
}(Endpoint));
exports.InEndpoint = InEndpoint;
/** Endpoints in the OUT direction (PC->device) have this type. */
var OutEndpoint = /** @class */ (function (_super) {
    __extends(OutEndpoint, _super);
    function OutEndpoint() {
        var _this = _super !== null && _super.apply(this, arguments) || this;
        /** Endpoint direction. */
        _this.direction = 'out';
        return _this;
    }
    /**
     * Perform a transfer to write `data` to the endpoint.
     *
     * If length is greater than maxPacketSize, libusb will automatically split the transfer in multiple packets, and you will receive one callback once all packets are complete.
     *
     * `this` in the callback is the OutEndpoint object.
     *
     * The device must be open to use this method.
     * @param buffer
     * @param callback
     */
    OutEndpoint.prototype.transfer = function (buffer, callback) {
        var _this = this;
        if (!buffer) {
            buffer = Buffer.alloc(0);
        }
        else if (!isBuffer(buffer)) {
            buffer = Buffer.from(buffer);
        }
        var cb = function (error, _buffer, actual) {
            if (callback) {
                callback.call(_this, error, actual || 0);
            }
        };
        try {
            this.makeTransfer(this.timeout, cb).submit(buffer);
        }
        catch (e) {
            process.nextTick(function () { return cb(e); });
        }
        return this;
    };
    OutEndpoint.prototype.transferWithZLP = function (buffer, callback) {
        if (buffer.length % this.descriptor.wMaxPacketSize === 0) {
            this.transfer(buffer);
            this.transfer(Buffer.alloc(0), callback);
        }
        else {
            this.transfer(buffer, callback);
        }
    };
    return OutEndpoint;
}(Endpoint));
exports.OutEndpoint = OutEndpoint;
//# sourceMappingURL=endpoint.js.map