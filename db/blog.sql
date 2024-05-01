/*
SQLyog Community v13.1.6 (64 bit)
MySQL - 8.2.0 : Database - hhzx_blog
*********************************************************************
*/

/*!40101 SET NAMES utf8 */;

/*!40101 SET SQL_MODE=''*/;

/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;
CREATE DATABASE /*!32312 IF NOT EXISTS*/`hhzx_blog` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;

USE `hhzx_blog`;

/*Table structure for table `article` */

DROP TABLE IF EXISTS `article`;

CREATE TABLE `article` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '文章ID，自增主键',
  `user_id` bigint DEFAULT NULL COMMENT '用户ID，外键引用user表，表示该文章的作者',
  `category_id` bigint DEFAULT NULL COMMENT '分类id',
  `article_tags` varchar(255) DEFAULT NULL COMMENT '所有标签',
  `title` varchar(255) NOT NULL COMMENT '文章标题，不能为空',
  `cover` varchar(255) DEFAULT NULL COMMENT '封面',
  `content` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '文章内容，纯文本格式，不能为空',
  `markdown` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '文章内容，Markdown格式，不能为空',
  `html` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '文章内容，HTML格式，不能为空',
  `publish_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '发布时间，默认当前时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间，默认当前时间',
  `status` tinyint(1) NOT NULL COMMENT '文章状态（0: 草稿，1: 已发布，2：审核），不能为空',
  `views` bigint DEFAULT '0' COMMENT '浏览量，默认为0',
  `likes` bigint DEFAULT '0' COMMENT '点赞数，默认为0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=15 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `article` */

insert  into `article`(`id`,`user_id`,`category_id`,`article_tags`,`title`,`cover`,`content`,`markdown`,`html`,`publish_time`,`update_time`,`status`,`views`,`likes`) values 
(2,1,13,'','编辑器测试',NULL,NULL,'## ? md-editor-v3\n\nMarkdown 编辑器，vue3 版本，使用 jsx 模板 和 typescript 开发，支持切换主题、prettier 美化文本等。\n\n### ? 基本演示\n\n**加粗**，<u>下划线</u>，_斜体_，~~删除线~~，上标<sup>26</sup>，下标<sub>1</sub>，`inline code`，[超链接](https://github.com/imzbf)\n\n> 引用：《I Have a Dream》\n\n1. So even though we face the difficulties of today and tomorrow, I still have a dream.\n2. It is a dream deeply rooted in the American dream.\n3. I have a dream that one day this nation will rise up.\n\n- [ ] 周五\n- [ ] 周六\n- [x] 周天\n\n![图片](https://imzbf.github.io/md-editor-rt/imgs/mark_emoji.gif)\n\n## ? 代码演示\n\n```vue\n<template>\n  <MdEditor v-model=\"text\" />\n</template>\n\n<script setup>\nimport { ref } from \'vue\';\nimport { MdEditor } from \'md-editor-v3\';\nimport \'md-editor-v3/lib/style.css\';\n\nconst text = ref(\'Hello Editor!\');\n</script>\n```\n\n## ? 文本演示\n\n依照普朗克长度这项单位，目前可观测的宇宙的直径估计值（直径约 930 亿光年，即 8.8 × 10<sup>26</sup> 米）即为 5.4 × 10<sup>61</sup>倍普朗克长度。而可观测宇宙体积则为 8.4 × 10<sup>184</sup>立方普朗克长度（普朗克体积）。\n\n## ? 表格演示\n\n| 昵称 | 来自      |\n| ---- | --------- |\n| 之间 | 中国-重庆 |\n\n## ? 公式\n\n行内：$x+y^{2x}$\n\n$$\n\\sqrt[3]{x}\n$$\n\n## ? 图表\n\n```mermaid\nflowchart TD\n  Start --> Stop\n```\n\n## ? 提示\n\n!!! note 支持的类型\n\nnote、abstract、info、tip、success、question、warning、failure、danger、bug、example、quote、hint、caution、error、attention\n\n!!!\n\n## ☘️ 占个坑@！',NULL,'2024-04-29 23:23:52','2024-04-29 23:23:52',0,5,0),
(4,1,13,'生活','文章1',NULL,NULL,'![](https://127.0.0.1:8888/file/download/MjA2CW2iZGB43CrN3T6SO-image.jpeg)\n',NULL,'2024-04-30 22:35:59','2024-04-30 22:35:59',0,6,0),
(5,1,13,'代码','图片1',NULL,NULL,'![](https://127.0.0.1:8888/file/download/4DBAbKu8WSqN4tARnxPy6-image.png)\n',NULL,'2024-04-30 22:38:04','2024-04-30 22:38:04',0,3,0),
(6,1,15,'','rust',NULL,NULL,'![](https://127.0.0.1:8888/file/download/TaL_cQRJiv3Haqr-Xmb1S-image.gif)\n',NULL,'2024-04-30 22:46:52','2024-04-30 22:46:52',0,0,0),
(7,1,15,'','梵高的星空',NULL,NULL,'![](https://127.0.0.1:8888/file/download/ZDZuJ-hamO4jmZaBP6XuR-image.jpeg)\n',NULL,'2024-04-30 22:47:35','2024-04-30 22:47:35',0,5,0),
(8,1,13,'','区域剪切',NULL,NULL,'![](https://127.0.0.1:8888/file/download/a_PFCDRG96st3JJHpHL1J-image.png)\n\n',NULL,'2024-04-30 22:50:29','2024-04-30 22:50:29',0,2,0),
(9,1,13,'','1',NULL,NULL,'![](https://127.0.0.1:8888/file/download/3rx7HO0Rf7M83t89gwr6B-image.jpeg)\n',NULL,'2024-05-01 13:13:17','2024-05-01 13:13:17',0,1,0),
(10,1,13,'','a1',NULL,NULL,'1',NULL,'2024-05-01 13:38:18','2024-05-01 13:38:18',0,3,0),
(11,1,13,'','b1',NULL,NULL,'a1',NULL,'2024-05-01 13:38:37','2024-05-01 13:38:37',0,4,0),
(13,1,13,'标签1,标签2,标签3','测100',NULL,NULL,'a',NULL,'2024-05-01 16:14:47','2024-05-01 16:27:29',0,11,0),
(14,1,13,'涩涩','图片',NULL,NULL,'# 雪女紧闭\n![](https://127.0.0.1:8888/file/download/v-dXzLG3O0FEK3_ZlzEDo-image.jpeg)\n\n# 雪女张开\n![](https://127.0.0.1:8888/file/download/O596HNx1AFJg0oFZ7U7dR-image.jpeg)\n',NULL,'2024-05-02 00:17:05','2024-05-02 00:17:05',0,49,0);

/*Table structure for table `article_tags` */

DROP TABLE IF EXISTS `article_tags`;

CREATE TABLE `article_tags` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `article_id` bigint NOT NULL COMMENT '文章ID',
  `tag_id` bigint NOT NULL COMMENT '标签ID',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `article_tags` */

/*Table structure for table `category` */

DROP TABLE IF EXISTS `category`;

CREATE TABLE `category` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '分类ID，自增主键',
  `category_name` varchar(50) NOT NULL COMMENT '分类名，不能为空',
  `ioc` varchar(50) DEFAULT NULL COMMENT '图标',
  `create_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间，默认当前时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间，默认当前时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=16 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `category` */

insert  into `category`(`id`,`category_name`,`ioc`,`create_time`,`update_time`) values 
(13,'默认',NULL,'2024-04-29 13:37:37','2024-04-29 13:37:37'),
(14,'代码',NULL,'2024-04-30 22:41:46','2024-04-30 22:41:46'),
(15,'生活',NULL,'2024-04-30 22:41:52','2024-04-30 22:41:52');

/*Table structure for table `comment` */

DROP TABLE IF EXISTS `comment`;

CREATE TABLE `comment` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '评论ID，自增主键',
  `comment_id` bigint DEFAULT NULL COMMENT '父留言id',
  `article_id` bigint NOT NULL COMMENT '文章ID，外键引用article表，表示该评论所属的文章',
  `user_id` bigint DEFAULT NULL COMMENT '用户ID，外键引用user表，表示该评论的作者',
  `qq` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'qq号',
  `web_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '网址',
  `content` text NOT NULL COMMENT '评论内容，不能为空',
  `create_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间，默认当前时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间，默认当前时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `comment` */

insert  into `comment`(`id`,`comment_id`,`article_id`,`user_id`,`qq`,`web_url`,`content`,`create_time`,`update_time`) values 
(1,NULL,14,NULL,'2831828656','http://blog.hhzx.top','留言','2024-05-02 00:25:45','2024-05-02 00:25:45'),
(2,1,14,NULL,'2831828656','http://blog.hhzx.top','回复','2024-05-02 00:59:58','2024-05-02 00:59:58'),
(3,1,14,NULL,'2402979195','1','回复','2024-05-02 01:01:40','2024-05-02 01:01:40'),
(4,1,14,NULL,'652034297',NULL,'不错','2024-05-02 01:05:10','2024-05-02 01:05:10');

/*Table structure for table `permissions` */

DROP TABLE IF EXISTS `permissions`;

CREATE TABLE `permissions` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '权限ID',
  `permission_name` varchar(255) NOT NULL COMMENT '权限名',
  `description` varchar(255) DEFAULT NULL COMMENT '权限描述',
  PRIMARY KEY (`id`),
  UNIQUE KEY `permission_name` (`permission_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `permissions` */

/*Table structure for table `role_permissions` */

DROP TABLE IF EXISTS `role_permissions`;

CREATE TABLE `role_permissions` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `role_id` bigint NOT NULL COMMENT '角色ID',
  `permission_id` bigint NOT NULL COMMENT '权限ID',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `role_permissions` */

/*Table structure for table `roles` */

DROP TABLE IF EXISTS `roles`;

CREATE TABLE `roles` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '角色ID',
  `role_name` varchar(255) NOT NULL COMMENT '角色名',
  `description` varchar(255) DEFAULT NULL COMMENT '角色描述',
  PRIMARY KEY (`id`),
  UNIQUE KEY `role_name` (`role_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `roles` */

/*Table structure for table `tags` */

DROP TABLE IF EXISTS `tags`;

CREATE TABLE `tags` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '标签ID',
  `tag_name` varchar(255) NOT NULL COMMENT '标签名',
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `tag_name` (`tag_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `tags` */

/*Table structure for table `user` */

DROP TABLE IF EXISTS `user`;

CREATE TABLE `user` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '用户ID，自增主键',
  `username` varchar(50) NOT NULL COMMENT '用户名，唯一且不能为空',
  `password` varchar(255) NOT NULL COMMENT '密码，应进行加密存储',
  `email` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '邮箱，唯一且不能为空',
  `nickname` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT '用户' COMMENT '昵称，不能为空',
  `avatar` varchar(255) DEFAULT NULL COMMENT '头像，可为空',
  `create_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间，默认当前时间',
  `update_time` datetime DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间，默认当前时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `username` (`username`),
  UNIQUE KEY `email` (`email`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `user` */

insert  into `user`(`id`,`username`,`password`,`email`,`nickname`,`avatar`,`create_time`,`update_time`) values 
(1,'hhzx','123456','2831828656@qq.com','用户','无','2024-04-27 18:08:29','2024-04-27 18:08:29');

/*Table structure for table `user_roles` */

DROP TABLE IF EXISTS `user_roles`;

CREATE TABLE `user_roles` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `user_id` bigint NOT NULL COMMENT '用户ID',
  `role_id` bigint NOT NULL COMMENT '角色ID',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

/*Data for the table `user_roles` */

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
