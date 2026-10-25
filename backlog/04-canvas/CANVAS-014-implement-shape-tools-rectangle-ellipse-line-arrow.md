---
id: CANVAS-014
title: Implement shape tools: rectangle, ellipse, line, arrow, polygon, star
tags: [canvas, rust, feature]
epic: CANVAS
priority: P0
depends_on: [CANVAS-008]
---

## Description

Draw basic shapes with Shift for squares/circles and Alt from center. Rectangles support independent corner radii and corner smoothing; polygons and stars have point count and inner radius; arrows have configurable heads.

## Acceptance criteria

- [ ] Each shape tool draws its shape with Shift and Alt modifiers
- [ ] Rectangle corner radii are editable per corner, with smoothing
- [ ] Polygon and star point count and ratio are editable
- [ ] Line and arrow endpoints and arrowhead styles are editable
