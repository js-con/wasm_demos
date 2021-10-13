/*
 * @Author: Lin Bowen
 * @Date: 2021-10-13 16:51:50
 * @LastEditTime: 2021-10-13 18:20:28
 * @LastEditors: Lin Bowen
 * @Description:
 * @FilePath: \wasm_demos\wasm-rust-md5-demo\src\utils\maze.ts
 */

import { CanvasHTMLAttributes } from "@vue/runtime-dom";

export class UnionSet {
  public set = null;
  constructor(size: number) {
    this.set = new Array(size);
    for (var i = this.set.length - 1; i >= 0; i--) {
      this.set[i] = -1;
    }
  }
  union(root1: number, root2: number) {
    if (this.set[root1] < this.set[root2]) {
      this.set[root2] = root1;
    } else {
      if (this.set[root1] === this.set[root2]) {
        this.set[root2]--;
      }
      this.set[root1] = root2;
    }
  }
  findSet(x: number) {
    if (this.set[x] < 0) return x;
    return (this.set[x] = this.findSet(this.set[x]));
  }
  sameSet(x: number, y: number) {
    return this.findSet(x) === this.findSet(y);
  }
  unionElement(x: number, y: number) {
    this.union(this.findSet(x), this.findSet(y));
  }
}
export class Maze {
  private columns: number;
  private rows: number;
  private cells: number;
  private linkedMap: {
    [key: string]: Array<number>;
  };
  private unionSets: UnionSet;
  private canvas: HTMLCanvasElement;
  constructor(columns, rows, canvas) {
    this.columns = columns;
    this.rows = rows;
    this.cells = columns * rows;
    //存放是连通的格子，{1: [2, 11]}表示第1个格子和第2、11个格子是相通的
    this.linkedMap = {};
    this.unionSets = new UnionSet(this.cells);
    this.canvas = canvas;
  }
  //取出随机的两个挨着的格子
  pickRandomCellPairs() {
    var cell = (Math.random() * this.cells) >> 0;
    //再取一个相邻格子，0 = 上，1 = 右，2 = 下，3 = 左
    var neiborCells = [];
    var row = (cell / this.columns) >> 0,
      column = cell % this.rows;
    //不是第一排的有上方的相邻元素
    if (row !== 0) {
      neiborCells.push(cell - this.columns);
    }
    //不是最后一排的有下面的相邻元素
    if (row !== this.rows - 1) {
      neiborCells.push(cell + this.columns);
    }
    if (column !== 0) {
      neiborCells.push(cell - 1);
    }
    if (column !== this.columns - 1) {
      neiborCells.push(cell + 1);
    }
    var index = (Math.random() * neiborCells.length) >> 0;
    return [cell, neiborCells[index]];
  }
  linkedToFirstCell() {
    for (var i = 1; i < this.cells; i++) {
      if (!this.unionSets.sameSet(0, i)) return false;
    }
    return true;
  }
  generate() {
    //每次任意取两个相邻的格子，如果它们不在同一个连通集，
    //则拆掉中间的墙，让它们连在一起成为一个连通集
    while (!this.linkedToFirstCell()) {
      var cellPairs = this.pickRandomCellPairs();
      if (!this.unionSets.sameSet(cellPairs[0], cellPairs[1])) {
        this.unionSets.unionElement(cellPairs[0], cellPairs[1]);
        this.addLinkedMap(cellPairs[0], cellPairs[1]);
      }
    }
  }
  addLinkedMap(x, y) {
    if (!this.linkedMap[x]) this.linkedMap[x] = [];
    if (!this.linkedMap[y]) this.linkedMap[y] = [];
    if (this.linkedMap[x].indexOf(y) < 0) {
      this.linkedMap[x].push(y);
    }
    if (this.linkedMap[y].indexOf(x) < 0) {
      this.linkedMap[y].push(x);
    }
  }
  draw() {
    //先画在缓存里，最后一次性绘制，提高性能
    var canvasBuffer = document.createElement("canvas");
    canvasBuffer.width = this.canvas.width;
    canvasBuffer.height = this.canvas.height;
    const cellWidth = canvasBuffer.width / this.columns,cellHeight = canvasBuffer.height / this.rows
    var ctx = canvasBuffer.getContext("2d");
    //translate 0.5个像素，避免模糊
    ctx.translate(0.5, 0.5);
    for (var i = 0; i < this.cells; i++) {
      var row = (i / this.columns) >> 0,
        column = i % this.columns;
      //画右边的竖线
      if (
        column !== this.columns - 1 &&
        (!this.linkedMap[i] || this.linkedMap[i].indexOf(i + 1) < 0)
      ) {
        ctx.moveTo(((column + 1) * cellWidth) >> 0, (row * cellHeight) >> 0);
        ctx.lineTo(
          ((column + 1) * cellWidth) >> 0,
          ((row + 1) * cellHeight) >> 0
        );
      }
      //画下面的横线
      if (
        row !== this.rows - 1 &&
        (!this.linkedMap[i] || this.linkedMap[i].indexOf(i + this.columns) < 0)
      ) {
        ctx.moveTo((column * cellWidth) >> 0, ((row + 1) * cellHeight) >> 0);
        ctx.lineTo(
          ((column + 1) * cellWidth) >> 0,
          ((row + 1) * cellHeight) >> 0
        );
      }
    }
    //画边框
    //this.drawBorder(ctx, cellWidth, cellHeight);
    ctx.moveTo(0,0)
    ctx.lineTo(canvasBuffer.width - 0.5,0)
    ctx.lineTo(canvasBuffer.width -0.5,canvasBuffer.height - 0.5)
    ctx.lineTo(0,canvasBuffer.height - 0.5)
    ctx.lineTo(0,0)
    //一次性stroke，提高性能
    ctx.stroke();
    this.canvas.getContext("2d").drawImage(canvasBuffer, 0, 0);
  }
}
